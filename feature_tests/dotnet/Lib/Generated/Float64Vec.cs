// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures;

#nullable enable

public partial class Float64Vec: IDisposable
{
    private unsafe Raw.Float64Vec* _inner;

    /// <summary>
    /// Creates a managed <c>Float64Vec</c> from a raw handle.
    /// </summary>
    /// <remarks>
    /// Safety: you should not build two managed objects using the same raw handle (may causes use-after-free and double-free).
    /// <br/>
    /// This constructor assumes the raw struct is allocated on Rust side.
    /// If implemented, the custom Drop implementation on Rust side WILL run on destruction.
    /// </remarks>
    public unsafe Float64Vec(Raw.Float64Vec* handle)
    {
        _inner = handle;
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec New(double[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (double* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.New(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewBool(bool[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (bool* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewBool(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewI16(short[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (short* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewI16(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewU16(ushort[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (ushort* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewU16(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewIsize(nint[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (nint* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewIsize(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewUsize(nuint[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (nuint* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewUsize(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewF64BeBytes(byte[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (byte* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewF64BeBytes(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    /// <returns>
    /// A <c>Float64Vec</c> allocated on Rust side.
    /// </returns>
    public static Float64Vec NewFromOwned(double[] v)
    {
        unsafe
        {
            nuint vLength = (nuint)v.Length;
            fixed (double* vPtr = v)
            {
                Raw.Float64Vec* retVal = Raw.Float64Vec.NewFromOwned(vPtr, vLength);
                return new Float64Vec(retVal);
            }
        }
    }

    public double[] AsBoxedSlice()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            Raw.double[] retVal = Raw.Float64Vec.AsBoxedSlice(_inner);
            return expected named type name, found `Box<[f64]>`;
        }
    }

    public double[] AsSlice()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            Raw.double[] retVal = Raw.Float64Vec.AsSlice(_inner);
            return expected named type name, found `&'a [f64]`;
        }
    }

    public void FillSlice(double[] v)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            nuint vLength = (nuint)v.Length;
            fixed (double* vPtr = v)
            {
                Raw.Float64Vec.FillSlice(_inner, vPtr, vLength);
            }
        }
    }

    public void SetValue(double[] newSlice)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            nuint newSliceLength = (nuint)newSlice.Length;
            fixed (double* newSlicePtr = newSlice)
            {
                Raw.Float64Vec.SetValue(_inner, newSlicePtr, newSliceLength);
            }
        }
    }

    public void ToString(DiplomatWriteable w)
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            Raw.Float64Vec.ToString(_inner, &w);
        }
    }

    public string ToString()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            DiplomatWriteable writeable = new DiplomatWriteable();
            Raw.Float64Vec.ToString(_inner, &writeable);
            string retVal = writeable.ToUnicode();
            writeable.Dispose();
            return retVal;
        }
    }

    public double[] Borrow()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("Float64Vec");
            }
            Raw.double[] retVal = Raw.Float64Vec.Borrow(_inner);
            return expected named type name, found `&'a [f64]`;
        }
    }

    /// <summary>
    /// Returns the underlying raw handle.
    /// </summary>
    public unsafe Raw.Float64Vec* AsFFI()
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

            Raw.Float64Vec.Destroy(_inner);
            _inner = null;

            GC.SuppressFinalize(this);
        }
    }

    ~Float64Vec()
    {
        Dispose();
    }
}
