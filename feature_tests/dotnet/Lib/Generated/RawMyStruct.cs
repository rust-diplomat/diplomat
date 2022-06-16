// Automatically generated by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct MyStruct
{
    private const string NativeLib = "diplomat_feature_tests";

    public byte a;

    [MarshalAs(UnmanagedType.U1)]
    public bool b;

    public byte c;

    public ulong d;

    public int e;

    public uint f;

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyStruct_new", ExactSpelling = true)]
    public static unsafe extern MyStruct New();

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "MyStruct_consume", ExactSpelling = true)]
    public static unsafe extern void Consume(MyStruct self);
}
