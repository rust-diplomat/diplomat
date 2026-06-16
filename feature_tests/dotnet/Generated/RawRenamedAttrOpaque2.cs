using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RenamedAttrOpaque2
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "namespace_AttrOpaque2_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RenamedAttrOpaque2* handle);
}