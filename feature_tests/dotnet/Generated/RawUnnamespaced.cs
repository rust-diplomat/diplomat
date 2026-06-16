using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Unnamespaced
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_Unnamespaced_make", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern Unnamespaced* Make(RenamedAttrEnum e);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_Unnamespaced_use_namespaced", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void UseNamespaced(Unnamespaced* handle, AttrOpaque1Renamed* n);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_Unnamespaced_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Unnamespaced* handle);
}