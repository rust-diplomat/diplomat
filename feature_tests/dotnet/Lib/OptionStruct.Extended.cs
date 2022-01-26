using System;
using System.Runtime.InteropServices;

namespace DiplomatFeatures;

public partial class OptionStruct
{
    public bool AIsNull()
    {
        unsafe
        {
            return AsFFI()->a == null;
        }
    }

    public bool BIsNull()
    {
        unsafe
        {
            return AsFFI()->b == null;
        }
    }

    public bool DIsNull()
    {
        unsafe
        {
            return AsFFI()->d == null;
        }
    }

    public void AssertIntegerForA(int i)
    {
        unsafe
        {
            Raw.OptionOpaque.AssertInteger(AsFFI()->a, i);
        }
    }

    public void AssertCharForB(uint ch)
    {
        unsafe
        {
            Raw.OptionOpaqueChar.AssertChar(AsFFI()->b, ch);
        }
    }

    public void AssertIntegerForD(int i)
    {
        unsafe
        {
            Raw.OptionOpaque.AssertInteger(AsFFI()->d, i);
        }
    }
}