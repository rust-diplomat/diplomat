using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultVoidResultOpaque
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal ResultOpaque* err;
    }

    private InnerUnion _inner;

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
    public unsafe ResultOpaque* Err => !IsOk ? _inner.err : throw new InvalidOperationException("Result does not contain Err value");
}