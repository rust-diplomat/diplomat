using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct ExclusiveWriter
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveWriter_add", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Add(ExclusiveWriter* handle, ulong delta);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "ExclusiveWriter_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(ExclusiveWriter* handle);
}