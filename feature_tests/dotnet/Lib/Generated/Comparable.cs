// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures;

#nullable enable

public partial class Comparable: IDisposable
{
    private unsafe Raw.Comparable* _inner;

    /// <summary>
    /// Creates a managed <c>Comparable</c> from a raw handle.
    /// </summary>
    /// <remarks>
    /// Safety: you should not build two managed objects using the same raw handle (may causes use-after-free and double-free).
    /// <br/>
    /// This constructor assumes the raw struct is allocated on Rust side.
    /// If implemented, the custom Drop implementation on Rust side WILL run on destruction.
    /// </remarks>
    public unsafe Comparable(Raw.Comparable* handle)
    {
        _inner = handle;
    }

    /// <returns>
    /// A <c>Comparable</c> allocated on Rust side.
    /// </returns>
    public static Comparable New(byte int)
    {
        unsafe
        {
            Raw.Comparable* retVal = Raw.Comparable.New(int);
            return new Comparable(retVal);
        }
    }

    public sbyte Cmp(Comparable other)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Comparable");
            }
            Raw.Comparable* otherRaw;
            otherRaw = other.AsFFI();
            if (otherRaw == null)
            {
                throw new ObjectDisposedException("Comparable");
            }
            Raw.sbyte retVal = Raw.Comparable.Cmp(_inner, otherRaw);
            return expected named type name, found `Ordering`;
        }
    }

    /// <summary>
    /// Returns the underlying raw handle.
    /// </summary>
    public unsafe Raw.Comparable* AsFFI()
    {
        return _inner;
    }

    /// <summary>
    /// Destroys the underlying object immediately.
    /// </summary>
    public void Dispose()
    {
        unsafe
        {
            if (_inner == null)
            {
                return;
            }

            Raw.Comparable.Destroy(_inner);
            _inner = null;

            GC.SuppressFinalize(this);
        }
    }

    ~Comparable()
    {
        Dispose();
    }
}
