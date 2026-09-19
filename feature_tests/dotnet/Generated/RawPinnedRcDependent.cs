using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct PinnedRcDependent
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcDependent_source_checksum", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong SourceChecksum(PinnedRcDependent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcDependent_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcDependent_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcDependent_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(PinnedRcDependent* handle);
}