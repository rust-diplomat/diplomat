using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct One
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(One* handle);
}