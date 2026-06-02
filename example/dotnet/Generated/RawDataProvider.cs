using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
public partial struct DataProvider
{
#if __IOS__
    private const string NativeLib = "libsomelib.framework/libsomelib";
#else
    private const string NativeLib = "somelib";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_DataProvider_new_static_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern DataProvider* NewStatic();

    [DllImport(NativeLib, EntryPoint = "icu4x_DataProvider_returns_result_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern DiplomatResultVoidUnit ReturnsResult();

    [DllImport(NativeLib, EntryPoint = "icu4x_DataProvider_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void Destroy(DataProvider* handle);
}