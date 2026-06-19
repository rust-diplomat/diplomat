using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ResultOpaque
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultResultOpaqueErrorEnum New(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_failing_foo", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultResultOpaqueErrorEnum NewFailingFoo();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_failing_bar", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultResultOpaqueErrorEnum NewFailingBar();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_failing_unit", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultResultOpaqueUnit NewFailingUnit();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_failing_struct", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultResultOpaqueErrorStruct NewFailingStruct(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_in_err", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultVoidResultOpaque NewInErr(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_int", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultIntUnit NewInt(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_new_in_enum_err", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultErrorEnumResultOpaque NewInEnumErr(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_assert_integer", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void AssertInteger(ResultOpaque* handle, int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ResultOpaque_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ResultOpaque* handle);
}