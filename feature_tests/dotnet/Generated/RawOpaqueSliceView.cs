using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueSliceView
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_parse", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOpaqueSliceViewSliceParseError Parse(DiplomatSliceU8 data);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_parse_strict", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatResultOpaqueSliceViewSliceParseError ParseStrict(DiplomatSliceU8 data);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_wrap", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern OpaqueSliceView* Wrap(DiplomatSliceU8 data);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_length", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern uint Length(OpaqueSliceView* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_get", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern byte Get(OpaqueSliceView* handle, uint index);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_sum", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern uint Sum(OpaqueSliceView* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueSliceView_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueSliceView* handle);
}