using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BorrowChild
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowChild_get", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern uint Get(BorrowChild* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowChild_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BorrowChild* handle);
}