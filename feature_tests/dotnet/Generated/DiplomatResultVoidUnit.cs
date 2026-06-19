using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;

[StructLayout(LayoutKind.Sequential)]
internal partial struct DiplomatResultVoidUnit
{

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
}