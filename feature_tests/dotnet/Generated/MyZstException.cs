using System;

namespace Somelib;

/// <summary>
/// If <c>MyZst</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
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