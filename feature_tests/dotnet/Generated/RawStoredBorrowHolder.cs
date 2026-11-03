using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowHolder
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowHolder_store", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Store(StoredBorrowHolder* handle, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowHolder_view", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowHolder* View(StoredBorrowHolder* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowHolder_touch", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Touch(StoredBorrowHolder* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowHolder_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowHolder* handle);
}