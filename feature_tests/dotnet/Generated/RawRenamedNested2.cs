using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedNested2
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_Nested2_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedNested2* handle);
}