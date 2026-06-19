using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Bar
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Bar_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Bar* handle);
}