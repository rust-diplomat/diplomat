using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BorrowingError
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowingError_owner_first", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThin* OwnerFirst(BorrowingError* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowingError_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BorrowingError* handle);
}