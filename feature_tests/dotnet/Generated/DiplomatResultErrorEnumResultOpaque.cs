using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultErrorEnumResultOpaque
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal ErrorEnum ok;
        [FieldOffset(0)] internal ResultOpaque* err;
    }

    private InnerUnion _inner;

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
    public ErrorEnum Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
    public unsafe ResultOpaque* Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}