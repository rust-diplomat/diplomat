using System;

namespace Somelib;

public class ErrorStructException : Exception
{
    public ErrorStruct Inner { get; }

    public ErrorStructException(ErrorStruct inner) : base(
        $"ErrorStruct: {inner}"
    )
    {
        Inner = inner;
    }
}