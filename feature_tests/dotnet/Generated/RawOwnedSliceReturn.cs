using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OwnedSliceReturn
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OwnedSliceReturn_make_bytes", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOwnedSliceU8 MakeBytes(uint len);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OwnedSliceReturn_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OwnedSliceReturn* handle);
}