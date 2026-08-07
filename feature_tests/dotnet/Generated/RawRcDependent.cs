using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct RcDependent
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong Id(RcDependent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_source_id", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong SourceId(RcDependent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_make_dependent2", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern RcDependent2* MakeDependent2(RcDependent* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_reset_drop_stats", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropStats();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_drop_seq", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropSeq();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "RcDependent_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(RcDependent* handle);
}