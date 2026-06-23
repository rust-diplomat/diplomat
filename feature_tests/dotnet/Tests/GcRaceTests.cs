using System;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// GC object-lifetime regression. Once a native call reads the receiver's
// pointer the GC may finalize it (-> ~T() -> Destroy -> drop) mid-call — a
// UAF that the generated GC.KeepAlive(this) prevents. See
// https://learn.microsoft.com/dotnet/standard/unsafe-code/best-practices
//
// Needs optimized IL so the JIT drops the receiver at last use; the csproj
// sets <Optimize>true</Optimize> for that, so the default `dotnet test` (CI)
// reproduces it. Without it, Debug roots locals and the race can't surface.
public class GcRaceTests
{
    // AggressiveOptimization: Tier1's precise liveness drops `probe` at its
    // last use (the race); Tier0 would root it and hide the bug.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static ulong UnrootedSlowCall(ulong millis)
    {
        GcRaceProbe probe = GcRaceProbe.Create();
        return probe.DropsDuringSpin(millis);
    }

    [Fact]
    public void InstanceCall_DoesNotFinalizeReceiverMidCall()
    {
        // Race needs a GC to finalize the receiver inside the call window:
        // retry, bail on first catch.
        ulong worstDropsDuringCall = 0;
        for (int attempt = 0; attempt < 50 && worstDropsDuringCall == 0; attempt++)
        {
            // Drain prior stragglers so a counted drop is THIS receiver, not
            // leftover garbage (else false positive).
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();

            ulong drops = 0;
            var call = Task.Run(() => drops = UnrootedSlowCall(60));

            // Pressure the GC while the call is in flight.
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
