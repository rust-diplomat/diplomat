using System;

namespace Somelib;

public class ErrorEnumException : Exception
{
    public ErrorEnum Inner { get; }
    private readonly object[] _edges;

    public ErrorEnumException(ErrorEnum inner, params object[] edges) : base(
        $"ErrorEnum: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}