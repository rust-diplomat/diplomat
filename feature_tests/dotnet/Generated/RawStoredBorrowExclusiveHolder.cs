using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowExclusiveHolder
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowExclusiveHolder* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_store", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Store(StoredBorrowExclusiveHolder* handle, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_store_then_fail", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidUnit StoreThenFail(StoredBorrowExclusiveHolder* handle, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_store_and_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern uint StoreAndCount(StoredBorrowExclusiveHolder* handle, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_store_chain", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void StoreChain(StoredBorrowExclusiveHolder* handle, StoredBorrowApple* apple, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowExclusiveHolder_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowExclusiveHolder* handle);
}