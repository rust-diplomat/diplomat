using System;

namespace Somelib;

public class MyZstException : Exception
{
    public MyZst Inner { get; }
    private readonly object[] _edges;

    public MyZstException(MyZst inner, params object[] edges) : base(
        $"MyZst: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}