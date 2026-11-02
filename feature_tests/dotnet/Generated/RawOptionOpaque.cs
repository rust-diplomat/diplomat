using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct OptionOpaque
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_new", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern OptionOpaque* New(int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_new_none", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern OptionOpaque* NewNone();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_option_isize", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionNInt OptionIsize(OptionOpaque* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_option_usize", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionNUInt OptionUsize(OptionOpaque* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_option_i32", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionInt OptionI32(OptionOpaque* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_option_u32", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DiplomatOptionUInt OptionU32(OptionOpaque* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_assert_integer", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void AssertInteger(OptionOpaque* handle, int i);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_option_opaque_argument", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    internal static unsafe extern bool OptionOpaqueArgument(OptionOpaque* arg);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_accepts_option_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint AcceptsOptionStr(DiplomatOptionSliceU8 arg, byte sentinel);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_accepts_option_primitive", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern long AcceptsOptionPrimitive(DiplomatOptionSliceU32 arg, byte sentinel);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_accepts_option_diplomat_str", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern nuint AcceptsOptionDiplomatStr(DiplomatOptionSliceU8 arg, byte sentinel);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "OptionOpaque_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(OptionOpaque* handle);
}