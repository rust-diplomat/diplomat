using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ExclusiveView
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveView_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Value(ExclusiveView* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveView_add", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Add(ExclusiveView* handle, ulong delta);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveView_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ExclusiveView* handle);
}