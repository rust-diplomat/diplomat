using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
public unsafe struct DiplomatSliceMutU8
{
    public byte* Ptr;
    public nuint Len;
}