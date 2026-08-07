using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RcDependent2
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent2_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Id(RcDependent2* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent2_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent2_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent2_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent2_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RcDependent2* handle);
}