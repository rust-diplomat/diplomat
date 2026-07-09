using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;
using Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultFixedDecimalFormatterUnit
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal FixedDecimalFormatter* ok;
    }

    private InnerUnion _inner;

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
    public unsafe FixedDecimalFormatter* Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
}