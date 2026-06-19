using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RefListParameter
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RefListParameter_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RefListParameter* handle);
}