using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OpaqueMutexedString
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_from_usize", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern OpaqueMutexedString* FromUsize(nuint number);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_change", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Change(OpaqueMutexedString* handle, nuint number);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_get_len_and_add", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint GetLenAndAdd(OpaqueMutexedString* handle, nuint other);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_wrapper", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Utf16Wrap* Wrapper(OpaqueMutexedString* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_to_unsigned_from_unsigned", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ushort ToUnsignedFromUnsigned(OpaqueMutexedString* handle, ushort input);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OpaqueMutexedString_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OpaqueMutexedString* handle);
}