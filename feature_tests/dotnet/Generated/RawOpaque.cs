using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct Opaque
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Opaque* New();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_try_from_utf8", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Opaque* TryFromUtf8(DiplomatSliceU8 input);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_from_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Opaque* FromStr(DiplomatSliceU8 input);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_get_debug_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void GetDebugStr(Opaque* handle, DiplomatWriteable* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_assert_struct", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void AssertStruct(Opaque* handle, MyStruct s);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_returns_usize", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint ReturnsUsize();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_returns_imported", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ImportedStruct ReturnsImported();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "Opaque_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(Opaque* handle);
}