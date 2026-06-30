using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedMixinTest
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_MixinTest_hello", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Hello(DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_MixinTest_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedMixinTest* handle);
}