using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct FixedDecimalFormatter
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimalFormatter_try_new_mv1", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultFixedDecimalFormatterUnit TryNew(Locale* locale, DataProvider* provider, FixedDecimalFormatterOptions options);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimalFormatter_format_write_mv1", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void FormatWrite(FixedDecimalFormatter* handle, FixedDecimal* value, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimalFormatter_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(FixedDecimalFormatter* handle);
}