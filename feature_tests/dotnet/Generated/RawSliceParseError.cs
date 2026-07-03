using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct SliceParseError
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "SliceParseError_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(SliceParseError* handle);
}