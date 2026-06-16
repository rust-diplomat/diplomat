using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RefList
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RefList_node", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern RefList* Node(RefListParameter* data);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RefList_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RefList* handle);
}