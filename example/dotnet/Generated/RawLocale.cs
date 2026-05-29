using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
public partial struct Locale
{
#if __IOS__
    private const string NativeLib = "libsomelib.framework/libsomelib";
#else
    private const string NativeLib = "somelib";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_Locale_new_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern Locale* New(DiplomatSliceU8 name);

    [DllImport(NativeLib, EntryPoint = "icu4x_Locale_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void Destroy(Locale* handle);
}