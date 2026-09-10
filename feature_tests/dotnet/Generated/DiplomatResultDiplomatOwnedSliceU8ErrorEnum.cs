using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;
using Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultDiplomatOwnedSliceU8ErrorEnum
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal DiplomatOwnedSliceU8 ok;
        [FieldOffset(0)] internal ErrorEnum err;
    }

    private InnerUnion _inner;

    public DiplomatBool IsOk;
    public DiplomatOwnedSliceU8 Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
    public ErrorEnum Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}