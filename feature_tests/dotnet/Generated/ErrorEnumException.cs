using System;

namespace Somelib;

/// <summary>
/// If <c>ErrorEnum</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class ErrorEnumException : Exception
{
    public ErrorEnum Inner { get; }

    public ErrorEnumException(ErrorEnum inner) : base(
        $"ErrorEnum: {inner}"
    )
    {
        Inner = inner;
    }
}