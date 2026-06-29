using System;

namespace Somelib;

public class ErrorStructException : Exception
{
    public ErrorStruct Inner { get; }
    private readonly object[] _edges;

    public ErrorStructException(ErrorStruct inner, params object[] edges) : base(
        $"ErrorStruct: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}