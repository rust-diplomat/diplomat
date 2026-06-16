using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedVectorTest
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_VectorTest_new", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern RenamedVectorTest* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_VectorTest_len", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern nuint Len(RenamedVectorTest* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_VectorTest_get", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatOptionDouble Get(RenamedVectorTest* handle, nuint idx);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_VectorTest_push", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void Push(RenamedVectorTest* handle, double value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_VectorTest_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedVectorTest* handle);
}