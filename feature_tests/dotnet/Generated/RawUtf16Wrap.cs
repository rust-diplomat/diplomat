using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Utf16Wrap
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_get_debug_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void GetDebugStr(Utf16Wrap* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Utf16Wrap* handle);
}