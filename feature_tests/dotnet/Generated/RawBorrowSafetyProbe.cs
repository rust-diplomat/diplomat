using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BorrowSafetyProbe
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern BorrowSafetyProbe* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_reset_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_reset_shared_call", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetSharedCall();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_shared_call_entered", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool SharedCallEntered();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_release_shared_call", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ReleaseSharedCall();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_hold_shared", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void HoldShared(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_ping_shared", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool PingShared(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_reset_mutable_call", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetMutableCall();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_mutable_call_entered", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool MutableCallEntered();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_release_mutable_call", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ReleaseMutableCall();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_hold_mutable", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void HoldMutable(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_ping_mutable", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool PingMutable(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_view", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern BorrowSafetyProbe* View(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_view_mut", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern BorrowSafetyProbe* ViewMut(BorrowSafetyProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_borrow_static_from_optional", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatSliceU8 BorrowStaticFromOptional(BorrowSafetyProbe* first, BorrowSafetyProbe* second);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowSafetyProbe_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BorrowSafetyProbe* handle);
}