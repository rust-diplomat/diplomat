using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Utf16Wrap
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_from_utf16", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Utf16Wrap* FromUtf16(DiplomatSliceU16 input);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_get_debug_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void GetDebugStr(Utf16Wrap* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_borrow_cont", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatSliceU16 BorrowCont(Utf16Wrap* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Utf16Wrap_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Utf16Wrap* handle);
}