using System;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Runtime regression for the GC object-lifetime pitfall the SafeHandle
// migration fixes — see
// https://learn.microsoft.com/dotnet/standard/unsafe-code/best-practices
// (section "Assumptions about object lifetimes (finalizers, GC.KeepAlive)").
//
// An instance method extracts the native pointer and then runs; once the
// pointer is read, the managed receiver is no longer referenced, so the GC
// can finalize it (-> Destroy -> drop the Rust Box) WHILE the call is still in
// flight — a use-after-free. `GcRaceProbe.DropsDuringSpin` sleeps without
// touching `self` again and returns how many `GcRaceProbe`s were dropped
// during its own execution; a correctly-rooted receiver yields 0.
//
// RED on the pre-migration wrapper (raw `Raw.T* _inner` + `~T()` finalizer, no
// keep-alive): under GC pressure the receiver is finalized mid-call -> >= 1.
// GREEN after the SafeHandle migration: the generated body emits
// `GC.KeepAlive(this)` after the native call, so the receiver stays rooted for
// the call's duration -> 0.
//
// IMPORTANT — this only REPRODUCES under the optimizing JIT *and* optimized IL,
// i.e. run it as:
//     dotnet test ... -c Release -e DOTNET_TieredCompilation=0
// Debug IL (and the Tier0 quick-JIT used for early calls) conservatively keeps
// the receiver rooted for the method's whole scope, so under the default
// `dotnet test` (Debug) this test passes trivially — it does not catch the
// regression there. It is shipped as an executable reproducer / proof of the
// fix; it always passes on the current SafeHandle wrapper.
public class GcRaceTests
{
    // AggressiveOptimization forces this method straight to the optimizing
    // (Tier1) JIT, whose precise GC liveness drops `probe` at its last use —
    // the condition under which the receiver can be collected mid-call. Tier0
    // (the default for a few early calls) conservatively keeps locals rooted
    // and hides the bug.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static ulong UnrootedSlowCall(ulong millis)
    {
        GcRaceProbe probe = GcRaceProbe.Create();
        // `probe` is not referenced after this call returns, so on a wrapper
        // that doesn't keep `this` alive it becomes collectible during the
        // native spin.
        return probe.DropsDuringSpin(millis);
    }

    [Fact]
    public void InstanceCall_DoesNotFinalizeReceiverMidCall()
    {
        // Retry: the race needs the optimizing JIT to drop the receiver at its
        // last use AND a GC to collect+finalize it inside the call window, so
        // give it many attempts and bail as soon as one catches the bug.
        // Run with DOTNET_TieredCompilation=0 to force optimized codegen.
        ulong worstDropsDuringCall = 0;
        for (int attempt = 0; attempt < 50 && worstDropsDuringCall == 0; attempt++)
        {
            // Clear any straggler probe from a previous attempt BEFORE the
            // call starts, so a drop counted during the spin (measured from a
            // baseline taken inside the call) can only be THIS attempt's
            // receiver — not leftover garbage. Without this the global drop
            // counter yields false positives.
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();

            ulong drops = 0;
            var call = Task.Run(() => drops = UnrootedSlowCall(60));

            // Hammer the GC (collect + run finalizers) + allocate pressure
            // while the native call is in flight.
            while (!call.IsCompleted)
            {
                _ = new byte[256 * 1024];
                GC.Collect();
                GC.WaitForPendingFinalizers();
                Thread.Yield();
            }
            call.Wait();
            worstDropsDuringCall = Math.Max(worstDropsDuringCall, drops);
        }

        Assert.Equal(0ul, worstDropsDuringCall);
    }
}
