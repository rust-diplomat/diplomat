using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueThinVec
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_create_single", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThinVec* CreateSingle(int a, float b, DiplomatSliceU8 c);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_set_first_c", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern void SetFirstC(OpaqueThinVec* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_iter", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThinIter* Iter(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_len", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern nuint Len(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_get", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThin* Get(OpaqueThinVec* handle, nuint idx);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_first", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OpaqueThin* First(OpaqueThinVec* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueThinVec_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueThinVec* handle);
}