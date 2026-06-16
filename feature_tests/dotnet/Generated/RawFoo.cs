using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Foo
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Foo_new", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern Foo* New(DiplomatSliceU8 x);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Foo_get_bar", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern Bar* GetBar(Foo* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Foo_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Foo* handle);
}