using System;

namespace Somelib;

public class ErrorEnumException : Exception
{
    public ErrorEnum Inner { get; }

    public ErrorEnumException(ErrorEnum inner) : base(
        $"ErrorEnum: {inner}"
    )
    {
        Inner = inner;
    }
}