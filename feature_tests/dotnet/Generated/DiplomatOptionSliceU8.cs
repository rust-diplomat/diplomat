using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

// Mirrors Rust `DiplomatOption<DiplomatSlice<T>>` = `#[repr(C)] { union { ok: T, err: () }, is_ok: bool }`.
// A union with a ZST arm has T's layout, so a sequential { slice, flag } matches.
[StructLayout(LayoutKind.Sequential)]
internal struct DiplomatOptionSliceU8
{
    public DiplomatSliceU8 Value;
    public DiplomatBool IsSome;

    public static DiplomatOptionSliceU8 Some(DiplomatSliceU8 value) =>
        new DiplomatOptionSliceU8 { Value = value, IsSome = true };

    public static DiplomatOptionSliceU8 None => default(DiplomatOptionSliceU8);
}