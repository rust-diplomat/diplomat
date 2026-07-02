using System;

namespace Somelib;

public class ResultOpaqueException : Exception
{
    public ResultOpaque Inner { get; }
    private readonly object[] _edges;

    public ResultOpaqueException(ResultOpaque inner, params object[] edges) : base(
        $"ResultOpaque: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}