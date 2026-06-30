using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueMut
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMut_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern OpaqueMut* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMut_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueMut* handle);
}