using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DefaultDropProbe
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DefaultDropProbe_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DefaultDropProbe* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DefaultDropProbe_reset_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DefaultDropProbe_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DefaultDropProbe_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(DefaultDropProbe* handle);
}