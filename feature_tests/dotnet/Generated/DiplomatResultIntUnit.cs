using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;
using Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultIntUnit
{
    [StructLayout(LayoutKind.Explicit)]
    private unsafe struct InnerUnion
    {
        [FieldOffset(0)] internal int ok;
    }

    private InnerUnion _inner;

    public DiplomatBool IsOk;
    public int Ok => IsOk ? _inner.ok : throw new InvalidOperationException("Result does not contain Ok value");
}