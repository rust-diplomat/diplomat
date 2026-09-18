using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ExclusiveSource
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ExclusiveSource* Create(ulong value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Value(ExclusiveSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_set_value", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetValue(ExclusiveSource* handle, ulong value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_view", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ExclusiveView* View(ExclusiveSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_view_mut", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ExclusiveView* ViewMut(ExclusiveSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_take_writer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ExclusiveWriter* TakeWriter(ExclusiveSource* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveSource_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ExclusiveSource* handle);
}