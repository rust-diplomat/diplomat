using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueThin
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThin_a", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern int A(OpaqueThin* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThin_b", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern float B(OpaqueThin* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThin_c", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void C(OpaqueThin* handle, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThin_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueThin* handle);
}