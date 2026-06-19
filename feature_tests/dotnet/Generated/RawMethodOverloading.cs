using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct MethodOverloading
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MethodOverloading_from_int32", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern MethodOverloading* from(int v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MethodOverloading_from_int64", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern MethodOverloading* from(long v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MethodOverloading_from_uint32", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern MethodOverloading* from(uint v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MethodOverloading_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(MethodOverloading* handle);
}