using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RcFinalizerDependent
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerDependent_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Id(RcFinalizerDependent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerDependent_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerDependent_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerDependent_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerDependent_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RcFinalizerDependent* handle);
}