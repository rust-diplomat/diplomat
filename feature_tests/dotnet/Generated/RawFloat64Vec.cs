using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Float64Vec
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Float64Vec_new_f64_be_bytes", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Float64Vec* NewF64BeBytes(DiplomatSliceU8 v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Float64Vec_to_string", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ToString(Float64Vec* handle, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Float64Vec_get", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionDouble Get(Float64Vec* handle, nuint i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Float64Vec_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Float64Vec* handle);
}