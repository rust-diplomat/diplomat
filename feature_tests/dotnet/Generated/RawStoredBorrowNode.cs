using System;
using System.Runtime.InteropServices;
using Somelib;
using Somelib.Diplomat;

namespace Somelib.Raw;

[StructLayout(LayoutKind.Sequential)]
internal partial struct StoredBorrowNode
{

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_create", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowNode* Create();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_link", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Link(StoredBorrowNode* handle, StoredBorrowNode* next);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_adopt_link", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void AdoptLink(StoredBorrowNode* handle, StoredBorrowNode* other);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_view", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern StoredBorrowNode* View(StoredBorrowNode* handle);

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_reset_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void ResetDropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_drop_count", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern ulong DropCount();

    [DllImport(DiplomatNativeLib.Name, EntryPoint = "StoredBorrowNode_destroy", CallingConvention = CallingConvention.Cdecl)]
    internal static unsafe extern void Destroy(StoredBorrowNode* handle);
}