using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct DiplomatSliceU32
{
    public uint* Ptr;
    public nuint Len;
}