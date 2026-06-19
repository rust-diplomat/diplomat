using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OptionOpaqueChar
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaqueChar_assert_char", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void AssertChar(OptionOpaqueChar* handle, uint ch);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaqueChar_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OptionOpaqueChar* handle);
}