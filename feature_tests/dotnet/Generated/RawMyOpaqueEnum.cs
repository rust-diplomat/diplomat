using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct MyOpaqueEnum
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyOpaqueEnum_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern MyOpaqueEnum* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyOpaqueEnum_to_string", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ToString(MyOpaqueEnum* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyOpaqueEnum_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(MyOpaqueEnum* handle);
}