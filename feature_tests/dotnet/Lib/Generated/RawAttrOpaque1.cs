// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct AttrOpaque1
{
    private const string NativeLib = "diplomat_feature_tests";

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "namespace_AttrOpaque1_method", ExactSpelling = true)]
    public static unsafe extern byte NamespaceMethod(AttrOpaque1* self);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "renamed_in_c_only", ExactSpelling = true)]
    public static unsafe extern byte RenamedInCOnly(AttrOpaque1* self);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "namespace_AttrOpaque1_method_disabledcpp", ExactSpelling = true)]
    public static unsafe extern void NamespaceMethodDisabledcpp(AttrOpaque1* self);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "AttrOpaque1_destroy", ExactSpelling = true)]
    public static unsafe extern void Destroy(AttrOpaque1* self);
}
