using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct PropertyMarshals
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern PropertyMarshals* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_number", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern uint Number(PropertyMarshals* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_number", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetNumber(PropertyMarshals* handle, uint value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_choice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern DefaultEnum Choice(PropertyMarshals* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_choice", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetChoice(PropertyMarshals* handle, DefaultEnum value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_point", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern PrimitiveStruct Point(PropertyMarshals* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_point", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetPoint(PropertyMarshals* handle, PrimitiveStruct value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_held", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern Opaque* Held(PropertyMarshals* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_held", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetHeld(PropertyMarshals* handle, Opaque* value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_utf8_text", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Utf8Text(PropertyMarshals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_utf8_text", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetUtf8Text(PropertyMarshals* handle, DiplomatSliceU8 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_utf16_text", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Utf16Text(PropertyMarshals* handle, DiplomatWrite* writeable);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_set_utf16_text", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void SetUtf16Text(PropertyMarshals* handle, DiplomatSliceU16 value);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "PropertyMarshals_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(PropertyMarshals* handle);
}