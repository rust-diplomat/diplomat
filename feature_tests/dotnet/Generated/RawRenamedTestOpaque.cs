using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedTestOpaque
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_TestOpaque_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedTestOpaque* handle);
}