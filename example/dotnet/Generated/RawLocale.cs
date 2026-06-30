using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Locale
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_Locale_new_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Locale* New(DiplomatSliceU8 name);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "icu4x_Locale_destroy_mv1", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Locale* handle);
}