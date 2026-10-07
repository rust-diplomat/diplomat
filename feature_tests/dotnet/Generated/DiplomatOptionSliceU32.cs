using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

// Mirrors Rust `DiplomatOption<DiplomatSlice<T>>` = `#[repr(C)] { union { ok: T, err: () }, is_ok: bool }`.
// A union with a ZST arm has T's layout, so a sequential { slice, flag } matches.
[StructLayout(LayoutKind.Sequential)]
internal struct DiplomatOptionSliceU32
{
    public DiplomatSliceU32 Value;
    public DiplomatBool IsSome;

    public static DiplomatOptionSliceU32 Some(DiplomatSliceU32 value) =>
        new DiplomatOptionSliceU32 { Value = value, IsSome = true };

    public static DiplomatOptionSliceU32 None => default(DiplomatOptionSliceU32);
}