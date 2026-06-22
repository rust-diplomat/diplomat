using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct BorrowParent
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowParent_create", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern BorrowParent* Create(uint value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowParent_view", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern BorrowChild* View(BorrowParent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowParent_value", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern uint Value(BorrowParent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "BorrowParent_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(BorrowParent* handle);
}