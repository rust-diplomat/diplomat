// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct MyIterable
{
    private const string NativeLib = "diplomat_feature_tests";

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "namespace_MyIterable_new", ExactSpelling = true)]
    public static unsafe extern MyIterable* NamespaceNew(byte* x, nuint xSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "namespace_MyIterable_iter", ExactSpelling = true)]
    public static unsafe extern MyIterator* NamespaceIter(MyIterable* self);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "namespace_MyIterable_destroy", ExactSpelling = true)]
    public static unsafe extern void Destroy(MyIterable* self);
}
