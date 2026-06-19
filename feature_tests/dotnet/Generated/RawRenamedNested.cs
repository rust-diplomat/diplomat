using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedNested
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_Nested_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedNested* handle);
}