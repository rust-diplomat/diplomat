using System;

namespace Somelib;

/// <summary>
/// If <c>BorrowingError</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class BorrowingErrorException : Exception
{
    public BorrowingError Inner { get; }

    public BorrowingErrorException(BorrowingError inner) : base(
        $"BorrowingError: {inner}"
    )
    {
        Inner = inner;
    }
}