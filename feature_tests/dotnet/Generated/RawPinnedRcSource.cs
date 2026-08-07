using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct PinnedRcSource
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern PinnedRcSource* Create(DiplomatSliceU8 data);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_make_dependent", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern PinnedRcDependent* MakeDependent(PinnedRcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_drop_checksum", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropChecksum();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PinnedRcSource_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(PinnedRcSource* handle);
}