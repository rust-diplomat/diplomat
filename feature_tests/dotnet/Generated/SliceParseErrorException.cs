using System;

namespace Somelib;

/// <summary>
/// If <c>SliceParseError</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class SliceParseErrorException : Exception
{
    public SliceParseError Inner { get; }

    public SliceParseErrorException(SliceParseError inner) : base(
        $"SliceParseError: {inner}"
    )
    {
        Inner = inner;
    }
}