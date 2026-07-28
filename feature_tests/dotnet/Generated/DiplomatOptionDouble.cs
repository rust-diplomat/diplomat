using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;
using Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatOptionDouble
{
    [StructLayout(LayoutKind.Explicit)]
    private struct InnerUnion
    {
        [FieldOffset(0)] internal double value;
    }

    private InnerUnion _inner;

    public DiplomatBool IsSome;

    public double Value => IsSome ? _inner.value : throw new InvalidOperationException("Option is None");
}