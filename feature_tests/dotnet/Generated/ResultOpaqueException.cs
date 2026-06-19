using System;

namespace Somelib;

public class ResultOpaqueException : Exception
{
    public ResultOpaque Inner { get; }

    public ResultOpaqueException(ResultOpaque inner) : base(
        $"ResultOpaque: {inner}"
    )
    {
        Inner = inner;
    }
}