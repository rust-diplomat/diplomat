using System;

namespace Somelib;

/// <summary>
/// If <c>BorrowingError</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that dependency is retained by <c>Inner</c>'s
/// own native resource state (see <c>RustHandle.cs.jinja</c>) rather than by
/// this exception class — so no separate keep-alive plumbing is needed here.
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