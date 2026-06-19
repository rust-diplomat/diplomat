using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct AttrOpaque1Renamed
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_new_overload", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern AttrOpaque1Renamed* NewOverload(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_new", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern AttrOpaque1Renamed* totally_not_New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_mac_test", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern int MacTest();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_hello", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern int Hello();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_method", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern byte method_renamed(AttrOpaque1Renamed* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "renamed_on_abi_only", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern byte Abirenamed(AttrOpaque1Renamed* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_use_unnamespaced", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void UseUnnamespaced(AttrOpaque1Renamed* handle, Unnamespaced* un);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_use_namespaced", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void UseNamespaced(AttrOpaque1Renamed* handle, RenamedAttrEnum n);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque1_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(AttrOpaque1Renamed* handle);
}