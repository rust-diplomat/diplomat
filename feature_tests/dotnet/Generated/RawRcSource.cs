using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RcSource
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcSource* Create(ulong id);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Id(RcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_view", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcSource* View(RcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_view_mut", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcSource* ViewMut(RcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_maybe_view_mut", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcSource* MaybeViewMut(RcSource* handle, [MarshalAs(UnmanagedType.U1)] bool present);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_ping_mutable", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool PingMutable(RcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_make_dependent", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcDependent* MakeDependent(RcSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcSource_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RcSource* handle);
}