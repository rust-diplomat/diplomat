using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OptionString
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionString_new", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern OptionString* New(DiplomatSliceU8 diplomatStr);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionString_write", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern DiplomatResultVoidUnit Write(OptionString* handle, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionString_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OptionString* handle);
}