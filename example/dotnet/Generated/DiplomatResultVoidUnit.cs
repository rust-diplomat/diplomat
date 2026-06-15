using System;
using System.Runtime.InteropServices;

namespace Somelib.Raw;

using Somelib;

[StructLayout(LayoutKind.Sequential)]
public partial struct DiplomatResultVoidUnit
{

    [MarshalAs(UnmanagedType.U1)]
    public bool IsOk;
}