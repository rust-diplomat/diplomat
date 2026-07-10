using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct MyString
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern MyString* New(DiplomatSliceU8 v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_new_unsafe", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern MyString* NewUnsafe(DiplomatSliceU8 v);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_set_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetStr(MyString* handle, DiplomatSliceU8 newStr);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_get_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void GetStr(MyString* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_string_transform", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void StringTransform(DiplomatSliceU8 foo, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "MyString_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(MyString* handle);
}