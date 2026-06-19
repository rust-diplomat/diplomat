using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

[StructLayout(LayoutKind.Sequential)]
internal unsafe struct DiplomatSliceMutU32
{
    public uint* Ptr;
    public nuint Len;
}