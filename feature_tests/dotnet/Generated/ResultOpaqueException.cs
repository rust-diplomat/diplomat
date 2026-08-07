using System;

namespace Somelib;

/// <summary>
/// If <c>ResultOpaque</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that dependency is retained by <c>Inner</c>'s
/// own native resource state (see <c>RustHandle.cs.jinja</c>) rather than by
/// this exception class — so no separate keep-alive plumbing is needed here.
/// </summary>
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