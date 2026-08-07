using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DisposableDropProbe
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DisposableDropProbe_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DisposableDropProbe* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DisposableDropProbe_is_alive", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool IsAlive(DisposableDropProbe* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DisposableDropProbe_reset_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DisposableDropProbe_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "DisposableDropProbe_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(DisposableDropProbe* handle);
}