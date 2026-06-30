using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct FixedDecimal
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimal_new_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern FixedDecimal* New(int v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimal_multiply_pow10_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void MultiplyPow10(FixedDecimal* handle, short power);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimal_to_string_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultVoidUnit ToString(FixedDecimal* handle, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_FixedDecimal_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(FixedDecimal* handle);
}