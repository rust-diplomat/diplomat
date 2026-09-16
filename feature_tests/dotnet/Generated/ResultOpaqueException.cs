using System;

namespace Somelib;

/// <summary>
/// If <c>ResultOpaque</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class ResultOpaqueException : Exception
{
    public ResultOpaque Inner { get; }

    public ResultOpaqueException(ResultOpaque inner) : base(
        $"ResultOpaque: {inner}"
    )
    {
        Inner = inner;
    }
}