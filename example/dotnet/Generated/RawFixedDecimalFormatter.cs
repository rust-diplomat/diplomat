using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
public partial struct FixedDecimalFormatter
{
#if __IOS__
    private const string NativeLib = "libsomelib.framework/libsomelib";
#else
    private const string NativeLib = "somelib";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_try_new_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern DiplomatResultFixedDecimalFormatterUnit TryNew(Locale* locale, DataProvider* provider, FixedDecimalFormatterOptions options);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_format_write_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern void FormatWrite(FixedDecimalFormatter* handle, FixedDecimal* value, DiplomatWriteable* writeable);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void Destroy(FixedDecimalFormatter* handle);
}