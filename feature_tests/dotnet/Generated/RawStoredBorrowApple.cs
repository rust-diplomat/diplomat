using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowApple
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowApple_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowApple* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowApple_take", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Take(StoredBorrowApple* handle, StoredBorrowBanana* banana);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowApple_keep", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Keep(StoredBorrowApple* handle, StoredBorrowBanana* banana);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowApple_store_mut", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void StoreMut(StoredBorrowApple* handle, StoredBorrowTarget* target);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowApple_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowApple* handle);
}