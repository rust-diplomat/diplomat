using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct FixedDecimalFormatter
{
#if __IOS__
    private const string NativeLib = "libdiplomat_example.framework/libdiplomat_example";
#else
    private const string NativeLib = "diplomat_example";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_try_new_mv1", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultFixedDecimalFormatterUnit TryNew(Locale* locale, DataProvider* provider, FixedDecimalFormatterOptions options);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_format_write_mv1", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void FormatWrite(FixedDecimalFormatter* handle, FixedDecimal* value, DiplomatWriteable* writeable);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimalFormatter_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(FixedDecimalFormatter* handle);
}