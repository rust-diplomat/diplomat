using System;
using System.Runtime.InteropServices;

namespace DiplomatFeatures;

public partial class OptionStruct
{
    public bool AIsNull()
    {
        unsafe
        {
            return _inner.a == null;
        }
    }

    public bool BIsNull()
    {
        unsafe
        {
            return _inner.b == null;
        }
    }

    public bool DIsNull()
    {
        unsafe
        {
            return _inner.d == null;
        }
    }

    public void AssertIntegerForA(int i)
    {
        unsafe
        {
            Raw.OptionOpaque.AssertInteger(_inner.a, i);
        }
    }

    public void AssertCharForB(uint ch)
    {
        unsafe
        {
            Raw.OptionOpaqueChar.AssertChar(_inner.b, ch);
        }
    }

    public void AssertIntegerForD(int i)
    {
        unsafe
        {
            Raw.OptionOpaque.AssertInteger(_inner.d, i);
        }
    }
}
