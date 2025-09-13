#include "diplomat_nanobind_common.hpp"
#include <../src/nb_internals.h>  // Required for shimming

// Forward declarations for binding add functions

void add_CallbackTestingStruct_binding(nb::handle);
void add_CallbackWrapper_binding(nb::handle);
void add_ImportedStruct_binding(nb::handle);
void add_BorrowedFields_binding(nb::handle);
void add_BorrowedFieldsReturning_binding(nb::handle);
void add_BorrowedFieldsWithBounds_binding(nb::handle);
void add_NestedBorrowedFields_binding(nb::handle);
void add_OptionInputStruct_binding(nb::handle);
void add_ErrorStruct_binding(nb::handle);
void add_BigStructWithStuff_binding(nb::handle);
void add_CyclicStructA_binding(nb::handle);
void add_CyclicStructB_binding(nb::handle);
void add_CyclicStructC_binding(nb::handle);
void add_MyStruct_binding(nb::handle);
void add_MyStructContainingAnOption_binding(nb::handle);
void add_MyZst_binding(nb::handle);
void add_PrimitiveStruct_binding(nb::handle);
void add_ScalarPairWithPadding_binding(nb::handle);
void add_StructArithmetic_binding(nb::handle);
void add_StructWithSlices_binding(nb::handle);
void add_OptionStruct_binding(nb::handle);
void add_Unnamespaced_binding(nb::handle);
void add_CallbackHolder_binding(nb::handle);
void add_MutableCallbackHolder_binding(nb::handle);
void add_Bar_binding(nb::handle);
void add_Foo_binding(nb::handle);
void add_One_binding(nb::handle);
void add_OpaqueThin_binding(nb::handle);
void add_OpaqueThinIter_binding(nb::handle);
void add_OpaqueThinVec_binding(nb::handle);
void add_Two_binding(nb::handle);
void add_OptionOpaque_binding(nb::handle);
void add_OptionOpaqueChar_binding(nb::handle);
void add_OptionString_binding(nb::handle);
void add_ResultOpaque_binding(nb::handle);
void add_RefList_binding(nb::handle);
void add_RefListParameter_binding(nb::handle);
void add_Float64Vec_binding(nb::handle);
void add_Float64VecError_binding(nb::handle);
void add_MyString_binding(nb::handle);
void add_MyOpaqueEnum_binding(nb::handle);
void add_Opaque_binding(nb::handle);
void add_OpaqueMutexedString_binding(nb::handle);
void add_PrimitiveStructVec_binding(nb::handle);
void add_Utf16Wrap_binding(nb::handle);
void add_UnimportedEnum_binding(nb::handle);
void add_OptionEnum_binding(nb::handle);
void add_ErrorEnum_binding(nb::handle);
void add_ContiguousEnum_binding(nb::handle);
void add_DefaultEnum_binding(nb::handle);
void add_MyEnum_binding(nb::handle);
namespace nested::ns {
void add_Nested_binding(nb::handle);
}

namespace nested::ns2 {
void add_Nested_binding(nb::handle);
}

namespace ns {
void add_RenamedDeprecatedStruct_binding(nb::handle);
void add_RenamedStructWithAttrs_binding(nb::handle);
void add_RenamedTestMacroStruct_binding(nb::handle);
void add_AttrOpaque1Renamed_binding(nb::handle);
void add_RenamedAttrOpaque2_binding(nb::handle);
void add_RenamedComparable_binding(nb::handle);
void add_RenamedDeprecatedOpaque_binding(nb::handle);
void add_RenamedMyIndexer_binding(nb::handle);
void add_RenamedMyIterable_binding(nb::handle);
void add_RenamedMyIterator_binding(nb::handle);
void add_RenamedOpaqueArithmetic_binding(nb::handle);
void add_RenamedOpaqueIterable_binding(nb::handle);
void add_RenamedOpaqueIterator_binding(nb::handle);
void add_RenamedTestOpaque_binding(nb::handle);
void add_RenamedVectorTest_binding(nb::handle);
void add_RenamedAttrEnum_binding(nb::handle);
void add_RenamedDeprecatedEnum_binding(nb::handle);
}


// Nanobind does not usually support custom deleters, so we're shimming some of the machinery to add that ability.
// On module init, the dummy type will have the normal nanobind inst_dealloc function in the tp_dealloc slot, so we
// pull it out, store it here, and then call it in the tp_dealloc function we are shimming in to all our types.
// Our custom tp_dealloc function will call the tp_free function instead of `delete`, allowing us effectively to override
// the delete operator.
// See https://nanobind.readthedocs.io/en/latest/lowlevel.html#customizing-type-creation and
// https://github.com/wjakob/nanobind/discussions/932
void (*nb_tp_dealloc)(void *) = nullptr;

void diplomat_tp_dealloc(PyObject *self)
{
    using namespace nb::detail;
    PyTypeObject *tp = Py_TYPE(self);
    const type_data *t = nb_type_data(tp);

    nb_inst *inst = (nb_inst *)self;
    void *p = inst_ptr(inst);
    if (inst->destruct)
    {
        inst->destruct = false;
        check(t->flags & (uint32_t)type_flags::is_destructible,
              "nanobind::detail::inst_dealloc(\"%s\"): attempted to call "
              "the destructor of a non-destructible type!",
              t->name);
        if (t->flags & (uint32_t)type_flags::has_destruct)
            t->destruct(p);
    }
    if (inst->cpp_delete)
    {
        inst->cpp_delete = false;
        auto tp_free = (freefunc)(PyType_GetSlot(tp, Py_tp_free));
        (*tp_free)(p);
    }
    (*nb_tp_dealloc)(self);
}

struct _Dummy {};

NB_MODULE(somelib, somelib_mod)
{
    {
        nb::class_<_Dummy> dummy(somelib_mod, "__dummy__");
        nb_tp_dealloc = (void (*)(void *))nb::type_get_slot(dummy, Py_tp_dealloc);
    }

    nb::class_<std::monostate>(somelib_mod, "monostate")
        .def("__repr__", [](const std::monostate &)
             { return ""; })
        .def("__str__", [](const std::monostate &)
             { return ""; });
             

    // Module declarations
    nb::module_ somelib_nested_mod = somelib_mod.def_submodule("nested");
    nb::module_ somelib_nested_ns_mod = somelib_nested_mod.def_submodule("ns");
    nb::module_ somelib_nested_ns2_mod = somelib_nested_mod.def_submodule("ns2");
    nb::module_ somelib_ns_mod = somelib_mod.def_submodule("ns");
    // Add bindings
    add_CallbackTestingStruct_binding(somelib_mod);
    add_CallbackWrapper_binding(somelib_mod);
    add_ImportedStruct_binding(somelib_mod);
    add_BorrowedFields_binding(somelib_mod);
    add_BorrowedFieldsReturning_binding(somelib_mod);
    add_BorrowedFieldsWithBounds_binding(somelib_mod);
    add_NestedBorrowedFields_binding(somelib_mod);
    add_OptionInputStruct_binding(somelib_mod);
    add_ErrorStruct_binding(somelib_mod);
    add_BigStructWithStuff_binding(somelib_mod);
    add_CyclicStructA_binding(somelib_mod);
    add_CyclicStructB_binding(somelib_mod);
    add_CyclicStructC_binding(somelib_mod);
    add_MyStruct_binding(somelib_mod);
    add_MyStructContainingAnOption_binding(somelib_mod);
    add_MyZst_binding(somelib_mod);
    add_PrimitiveStruct_binding(somelib_mod);
    add_ScalarPairWithPadding_binding(somelib_mod);
    add_StructArithmetic_binding(somelib_mod);
    add_StructWithSlices_binding(somelib_mod);
    add_OptionStruct_binding(somelib_mod);
    add_Unnamespaced_binding(somelib_mod);
    add_CallbackHolder_binding(somelib_mod);
    add_MutableCallbackHolder_binding(somelib_mod);
    add_Bar_binding(somelib_mod);
    add_Foo_binding(somelib_mod);
    add_One_binding(somelib_mod);
    add_OpaqueThin_binding(somelib_mod);
    add_OpaqueThinIter_binding(somelib_mod);
    add_OpaqueThinVec_binding(somelib_mod);
    add_Two_binding(somelib_mod);
    add_OptionOpaque_binding(somelib_mod);
    add_OptionOpaqueChar_binding(somelib_mod);
    add_OptionString_binding(somelib_mod);
    add_ResultOpaque_binding(somelib_mod);
    add_RefList_binding(somelib_mod);
    add_RefListParameter_binding(somelib_mod);
    add_Float64Vec_binding(somelib_mod);
    add_Float64VecError_binding(somelib_mod);
    add_MyString_binding(somelib_mod);
    add_MyOpaqueEnum_binding(somelib_mod);
    add_Opaque_binding(somelib_mod);
    add_OpaqueMutexedString_binding(somelib_mod);
    add_PrimitiveStructVec_binding(somelib_mod);
    add_Utf16Wrap_binding(somelib_mod);
    add_UnimportedEnum_binding(somelib_mod);
    add_OptionEnum_binding(somelib_mod);
    add_ErrorEnum_binding(somelib_mod);
    add_ContiguousEnum_binding(somelib_mod);
    add_DefaultEnum_binding(somelib_mod);
    add_MyEnum_binding(somelib_mod);
    
    nested::ns::add_Nested_binding(somelib_nested_ns_mod);
    
    nested::ns2::add_Nested_binding(somelib_nested_ns2_mod);
    
    ns::add_RenamedDeprecatedStruct_binding(somelib_ns_mod);
    ns::add_RenamedStructWithAttrs_binding(somelib_ns_mod);
    ns::add_RenamedTestMacroStruct_binding(somelib_ns_mod);
    ns::add_AttrOpaque1Renamed_binding(somelib_ns_mod);
    ns::add_RenamedAttrOpaque2_binding(somelib_ns_mod);
    ns::add_RenamedComparable_binding(somelib_ns_mod);
    ns::add_RenamedDeprecatedOpaque_binding(somelib_ns_mod);
    ns::add_RenamedMyIndexer_binding(somelib_ns_mod);
    ns::add_RenamedMyIterable_binding(somelib_ns_mod);
    ns::add_RenamedMyIterator_binding(somelib_ns_mod);
    ns::add_RenamedOpaqueArithmetic_binding(somelib_ns_mod);
    ns::add_RenamedOpaqueIterable_binding(somelib_ns_mod);
    ns::add_RenamedOpaqueIterator_binding(somelib_ns_mod);
    ns::add_RenamedTestOpaque_binding(somelib_ns_mod);
    ns::add_RenamedVectorTest_binding(somelib_ns_mod);
    ns::add_RenamedAttrEnum_binding(somelib_ns_mod);
    ns::add_RenamedDeprecatedEnum_binding(somelib_ns_mod);
    
    
}