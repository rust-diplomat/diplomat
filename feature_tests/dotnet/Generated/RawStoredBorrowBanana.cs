using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowBanana
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowBanana_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowBanana* handle);
}