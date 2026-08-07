using System;

namespace Somelib;

/// <summary>
/// If <c>ErrorEnum</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that dependency is retained by <c>Inner</c>'s
/// own native resource state (see <c>RustHandle.cs.jinja</c>) rather than by
/// this exception class — so no separate keep-alive plumbing is needed here.
/// </summary>
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