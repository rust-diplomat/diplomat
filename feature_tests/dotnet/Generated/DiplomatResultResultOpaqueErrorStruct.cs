using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultResultOpaqueErrorStruct
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal ResultOpaque* ok;
        [FieldOffset(0)] internal ErrorStruct err;
    }

    private InnerUnion _inner;

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
    public unsafe ResultOpaque* Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
    public ErrorStruct Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}