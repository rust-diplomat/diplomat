using System;

namespace Somelib.Diplomat;

#nullable enable

public delegate void DiplomatBorrowedSpanAction<T>(ReadOnlySpan<T> span) where T : unmanaged;

/// <summary>
/// A zero-copy view over memory Rust still owns (a borrowed <c>&amp;str</c> /
/// <c>&amp;[T]</c> return). Unlike <c>RustVec</c>, there's nothing to free
/// here — Rust still owns this memory, so there's no <c>IDisposable</c> and
/// no ownership race to guard against. <c>edges</c> roots whatever this was
/// borrowed from (the receiver or an input parameter) so the GC can't
/// collect it while this view is alive.
/// </summary>
/// <remarks>
/// This intentionally does not expose a public <c>Span</c>-returning
/// property. A caller could extract it, let this value go, and be left
/// holding a span with nothing keeping <c>edges</c> (and the parent it
/// roots) alive — exactly the trap <c>RustVec</c> avoids by not implementing
/// <c>MemoryManager&lt;T&gt;</c>. <see cref="WithSpan"/> gives synchronous,
/// zero-copy, read-only access instead: the callback receives the span
/// directly, so it can never outlive this value's own lifetime. This is a
/// plain struct, not a <c>ref struct</c>, so it can be stored in a field, a
/// collection, or held across an <c>await</c> — unlike a bare
/// <see cref="ReadOnlySpan{T}"/>. <see cref="Clone"/> is the explicit,
/// independent copy — never something this type does on its own.
/// </remarks>
public readonly unsafe struct DiplomatBorrowedSpan<T> where T : unmanaged
{
    private readonly T* _ptr;
    private readonly int _len;
    private readonly object[] _edges;

    internal DiplomatBorrowedSpan(T* ptr, nuint len, object[] edges)
    {
        _ptr = ptr;
        _len = (int)len;
        _edges = edges;
    }

    public int Length => _len;

    /// <summary>
    /// Synchronous, zero-copy, read-only access. The span is valid only for
    /// the duration of this callback.
    /// </summary>
    public void WithSpan(DiplomatBorrowedSpanAction<T> action)
    {
        if (action is null)
        {
            throw new ArgumentNullException(nameof(action));
        }
        action(new ReadOnlySpan<T>(_ptr, _len));
        GC.KeepAlive(_edges);
    }

    /// <summary>An explicit, independent copy — never implicit.</summary>
    public T[] Clone() => new ReadOnlySpan<T>(_ptr, _len).ToArray();
}