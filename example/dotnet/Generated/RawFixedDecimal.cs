using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
public partial struct FixedDecimal
{
#if __IOS__
    private const string NativeLib = "libdiplomat_example.framework/libdiplomat_example";
#else
    private const string NativeLib = "diplomat_example";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimal_new_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern FixedDecimal* New(int v);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimal_multiply_pow10_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern void MultiplyPow10(FixedDecimal* handle, short power);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimal_to_string_mv1", CallingConvention = CallingConvention.Cdecl)]
public static unsafe extern DiplomatResultVoidUnit ToString(FixedDecimal* handle, DiplomatWriteable* writeable);

    [DllImport(NativeLib, EntryPoint = "icu4x_FixedDecimal_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    public static unsafe extern void Destroy(FixedDecimal* handle);
}