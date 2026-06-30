using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedOpaqueZSTIndexer
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_OpaqueZSTIndexer_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RenamedOpaqueZSTIndexer* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_OpaqueZSTIndexer_index", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RenamedOpaqueZSTIndexer* Index(RenamedOpaqueZSTIndexer* handle, nuint idx);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_OpaqueZSTIndexer_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedOpaqueZSTIndexer* handle);
}