// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct MyString
{
    private const string NativeLib = "diplomat_feature_tests";

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_new", ExactSpelling = true)]
    public static unsafe extern MyString* New(byte* v, nuint vSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_new_unsafe", ExactSpelling = true)]
    public static unsafe extern MyString* NewUnsafe(ushort* v, nuint vSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_new_owned", ExactSpelling = true)]
    public static unsafe extern MyString* NewOwned(byte* v, nuint vSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_set_str", ExactSpelling = true)]
    public static unsafe extern void SetStr(MyString* self, byte* newStr, nuint newStrSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_get_str", ExactSpelling = true)]
    public static unsafe extern void GetStr(MyString* self, DiplomatWrite* write);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyString_destroy", ExactSpelling = true)]
    public static unsafe extern void Destroy(MyString* self);
}
