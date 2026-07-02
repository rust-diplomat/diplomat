using System;

namespace Somelib;

public class BorrowingErrorException : Exception
{
    public BorrowingError Inner { get; }
    private readonly object[] _edges;

    public BorrowingErrorException(BorrowingError inner, params object[] edges) : base(
        $"BorrowingError: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}