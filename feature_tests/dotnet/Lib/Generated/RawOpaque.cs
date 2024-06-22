// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct Opaque
{
    private const string NativeLib = "diplomat_feature_tests";

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_new", ExactSpelling = true)]
    public static unsafe extern Opaque* New();

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_try_from_utf8", ExactSpelling = true)]
    public static unsafe extern Opaque* TryFromUtf8(byte* input, nuint inputSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_from_str", ExactSpelling = true)]
    public static unsafe extern Opaque* FromStr(ushort* input, nuint inputSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_get_debug_str", ExactSpelling = true)]
    public static unsafe extern void GetDebugStr(Opaque* self, DiplomatWrite* write);

    /// <summary>
    /// See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
    /// </summary>
    /// <remarks>
    /// See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
    /// <br/>
    /// Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
    /// </remarks>
    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_assert_struct", ExactSpelling = true)]
    public static unsafe extern void AssertStruct(Opaque* self, MyStruct s);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_returns_usize", ExactSpelling = true)]
    public static unsafe extern nuint ReturnsUsize();

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_returns_imported", ExactSpelling = true)]
    public static unsafe extern ImportedStruct ReturnsImported();

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_cmp", ExactSpelling = true)]
    public static unsafe extern sbyte Cmp();

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "Opaque_destroy", ExactSpelling = true)]
    public static unsafe extern void Destroy(Opaque* self);
}
