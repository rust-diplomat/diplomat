using System;

namespace Somelib;

/// <summary>
/// If <c>ErrorStruct</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class ErrorStructException : Exception
{
    public ErrorStruct Inner { get; }

    public ErrorStructException(ErrorStruct inner) : base(
        $"ErrorStruct: {inner}"
    )
    {
        Inner = inner;
    }
}