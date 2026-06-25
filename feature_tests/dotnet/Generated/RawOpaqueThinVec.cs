using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueThinVec
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_iter", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThinIter* Iter(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_len", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern nuint Len(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_first", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThin* First(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueThinVec* handle);
}