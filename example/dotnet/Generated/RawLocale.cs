using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Locale
{
#if __IOS__
    private const string NativeLib = "libdiplomat_example.framework/libdiplomat_example";
#else
    private const string NativeLib = "diplomat_example";
#endif

    [DllImport(NativeLib, EntryPoint = "icu4x_Locale_new_mv1", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern Locale* New(DiplomatSliceU8 name);

    [DllImport(NativeLib, EntryPoint = "icu4x_Locale_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Locale* handle);
}