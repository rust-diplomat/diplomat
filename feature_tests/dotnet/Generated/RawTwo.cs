using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Two
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Two_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Two* handle);
}