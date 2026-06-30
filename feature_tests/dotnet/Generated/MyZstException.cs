using System;

namespace Somelib;

public class MyZstException : Exception
{
    public MyZst Inner { get; }

    public MyZstException(MyZst inner) : base(
        $"MyZst: {inner}"
    )
    {
        Inner = inner;
    }
}