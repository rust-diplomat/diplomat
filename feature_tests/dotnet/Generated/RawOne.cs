using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct One
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_transitivity", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* Transitivity(One* hold, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_cycle", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* Cycle(Two* hold, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_many_dependents", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* ManyDependents(One* a, One* b, Two* c, Two* d, Two* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_return_outlives_param", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* ReturnOutlivesParam(Two* hold, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_diamond_top", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* DiamondTop(One* top, One* left, One* right, One* bottom);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_diamond_left", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* DiamondLeft(One* top, One* left, One* right, One* bottom);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_diamond_right", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* DiamondRight(One* top, One* left, One* right, One* bottom);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_diamond_bottom", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* DiamondBottom(One* top, One* left, One* right, One* bottom);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_diamond_and_nested_types", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* DiamondAndNestedTypes(One* a, One* b, One* c, One* d, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_implicit_bounds", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* ImplicitBounds(One* explicitHold, One* implicitHold, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_implicit_bounds_deep", CallingConvention = CallingConvention.Cdecl)]
internal static unsafe extern One* ImplicitBoundsDeep(One* @explicit, One* implicit1, One* implicit2, One* nohold);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "One_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(One* handle);
}