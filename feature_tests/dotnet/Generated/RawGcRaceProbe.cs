using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct GcRaceProbe
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "GcRaceProbe_create", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern GcRaceProbe* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "GcRaceProbe_drops_during_spin", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern ulong DropsDuringSpin(GcRaceProbe* handle, ulong millis);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "GcRaceProbe_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(GcRaceProbe* handle);
}