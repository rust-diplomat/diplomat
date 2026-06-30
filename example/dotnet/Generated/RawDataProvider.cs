using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DataProvider
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_DataProvider_new_static_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DataProvider* NewStatic();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_DataProvider_returns_result_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidUnit ReturnsResult();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_DataProvider_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(DataProvider* handle);
}