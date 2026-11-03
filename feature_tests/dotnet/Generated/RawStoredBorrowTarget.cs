using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowTarget
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowTarget* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_make_holder", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowHolder* MakeHolder(StoredBorrowTarget* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_make_banana", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowBanana* MakeBanana(StoredBorrowTarget* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_lend_to", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowBanana* LendTo(StoredBorrowTarget* handle, StoredBorrowHolder* holder);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_touch", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Touch(StoredBorrowTarget* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_reset_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowTarget_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowTarget* handle);
}