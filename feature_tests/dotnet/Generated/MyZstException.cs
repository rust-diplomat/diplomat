using System;

namespace Somelib;

/// <summary>
/// If <c>MyZst</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that dependency is retained by <c>Inner</c>'s
/// own native resource state (see <c>RustHandle.cs.jinja</c>) rather than by
/// this exception class — so no separate keep-alive plumbing is needed here.
/// </summary>
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