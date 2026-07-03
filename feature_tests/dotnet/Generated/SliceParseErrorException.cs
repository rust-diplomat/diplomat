using System;

namespace Somelib;

public class SliceParseErrorException : Exception
{
    public SliceParseError Inner { get; }
    private readonly object[] _edges;

    public SliceParseErrorException(SliceParseError inner, params object[] edges) : base(
        $"SliceParseError: {inner}"
    )
    {
        Inner = inner;
        _edges = edges;
    }
}