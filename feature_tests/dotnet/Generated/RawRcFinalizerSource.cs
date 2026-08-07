using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RcFinalizerSource
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcFinalizerSource* Create(ulong id);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Id(RcFinalizerSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_make_dependent", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcFinalizerDependent* MakeDependent(RcFinalizerSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcFinalizerSource_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RcFinalizerSource* handle);
}