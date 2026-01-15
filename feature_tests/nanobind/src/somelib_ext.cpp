#include "diplomat_nanobind_common.hpp"
#include <../src/nb_internals.h>  // Required for shimming

// Forward declarations for binding add functions
namespace somelib{
  
void add_CallbackTestingStruct_binding(nb::module_);
void add_CallbackWrapper_binding(nb::module_);
void add_ImportedStruct_binding(nb::module_);
void add_BorrowedFields_binding(nb::module_);
void add_BorrowedFieldsReturning_binding(nb::module_);
void add_BorrowedFieldsWithBounds_binding(nb::module_);
void add_NestedBorrowedFields_binding(nb::module_);
void add_BorrowingOptionStruct_binding(nb::module_);
void add_OptionInputStruct_binding(nb::module_);
void add_ErrorStruct_binding(nb::module_);
void add_BigStructWithStuff_binding(nb::module_);
void add_CyclicStructA_binding(nb::module_);
void add_CyclicStructB_binding(nb::module_);
void add_CyclicStructC_binding(nb::module_);
void add_MyStruct_binding(nb::module_);
void add_MyStructContainingAnOption_binding(nb::module_);
void add_MyZst_binding(nb::module_);
void add_PrimitiveStruct_binding(nb::module_);
void add_ScalarPairWithPadding_binding(nb::module_);
void add_StructArithmetic_binding(nb::module_);
void add_StructWithSlices_binding(nb::module_);
void add_OptionStruct_binding(nb::module_);
void add_Unnamespaced_binding(nb::module_);
void add_CallbackHolder_binding(nb::module_);
void add_MutableCallbackHolder_binding(nb::module_);
void add_Bar_binding(nb::module_);
void add_Foo_binding(nb::module_);
void add_One_binding(nb::module_);
void add_OpaqueThin_binding(nb::module_);
void add_OpaqueThinIter_binding(nb::module_);
void add_OpaqueThinVec_binding(nb::module_);
void add_Two_binding(nb::module_);
void add_OptionOpaque_binding(nb::module_);
void add_OptionOpaqueChar_binding(nb::module_);
void add_OptionString_binding(nb::module_);
void add_ResultOpaque_binding(nb::module_);
void add_RefList_binding(nb::module_);
void add_RefListParameter_binding(nb::module_);
void add_Float64Vec_binding(nb::module_);
void add_Float64VecError_binding(nb::module_);
void add_MyString_binding(nb::module_);
void add_MyOpaqueEnum_binding(nb::module_);
void add_Opaque_binding(nb::module_);
void add_OpaqueMutexedString_binding(nb::module_);
void add_PrimitiveStructVec_binding(nb::module_);
void add_Utf16Wrap_binding(nb::module_);
void add_UnimportedEnum_binding(nb::module_);
void add_OptionEnum_binding(nb::module_);
void add_ErrorEnum_binding(nb::module_);
void add_ContiguousEnum_binding(nb::module_);
void add_DefaultEnum_binding(nb::module_);
void add_MyEnum_binding(nb::module_);
void add_free_function_binding(nb::module_);
}namespace somelib::nested::ns{
  
void add_Nested_binding(nb::module_);
void add_free_function_binding(nb::module_);
}namespace somelib::nested::ns2{
  
void add_Nested_binding(nb::module_);
}namespace somelib::ns{
  
void add_RenamedDeprecatedStruct_binding(nb::module_);
void add_RenamedStructWithAttrs_binding(nb::module_);
void add_RenamedTestMacroStruct_binding(nb::module_);
void add_AttrOpaque1Renamed_binding(nb::module_);
void add_RenamedAttrOpaque2_binding(nb::module_);
void add_RenamedBlockOverride_binding(nb::module_);
void add_RenamedComparable_binding(nb::module_);
void add_RenamedDeprecatedOpaque_binding(nb::module_);
void add_RenamedMyIndexer_binding(nb::module_);
void add_RenamedMyIterable_binding(nb::module_);
void add_RenamedMyIterator_binding(nb::module_);
void add_RenamedOpaqueArithmetic_binding(nb::module_);
void add_RenamedOpaqueIterable_binding(nb::module_);
void add_RenamedOpaqueIterator_binding(nb::module_);
void add_RenamedOpaqueRefIterable_binding(nb::module_);
void add_RenamedOpaqueRefIterator_binding(nb::module_);
void add_RenamedStringList_binding(nb::module_);
void add_RenamedTestOpaque_binding(nb::module_);
void add_RenamedVectorTest_binding(nb::module_);
void add_RenamedAttrEnum_binding(nb::module_);
void add_RenamedDeprecatedEnum_binding(nb::module_);
void add_free_function_binding(nb::module_);
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

NB_MODULE(somelib, mod)
{
    using namespace somelib;

    {
        nb::class_<_Dummy> dummy(mod, "__dummy__");
        nb_tp_dealloc = (void (*)(void *))nb::type_get_slot(dummy, Py_tp_dealloc);
    }

    nb::class_<std::monostate>(mod, "monostate")
        .def("__repr__", [](const std::monostate &)
             { return ""; })
        .def("__str__", [](const std::monostate &)
             { return ""; });// Module declarations
    nb::module_ nested_mod = mod.def_submodule("nested");
    nb::module_ nested_ns_mod = nested_mod.def_submodule("ns");
    nb::module_ nested_ns2_mod = nested_mod.def_submodule("ns2");
    nb::module_ ns_mod = mod.def_submodule("ns");
    // Add bindings
    add_CallbackTestingStruct_binding(mod);
    add_CallbackWrapper_binding(mod);
    add_ImportedStruct_binding(mod);
    add_BorrowedFields_binding(mod);
    add_BorrowedFieldsReturning_binding(mod);
    add_BorrowedFieldsWithBounds_binding(mod);
    add_NestedBorrowedFields_binding(mod);
    add_BorrowingOptionStruct_binding(mod);
    add_OptionInputStruct_binding(mod);
    add_ErrorStruct_binding(mod);
    add_BigStructWithStuff_binding(mod);
    add_CyclicStructA_binding(mod);
    add_CyclicStructB_binding(mod);
    add_CyclicStructC_binding(mod);
    add_MyStruct_binding(mod);
    add_MyStructContainingAnOption_binding(mod);
    add_MyZst_binding(mod);
    add_PrimitiveStruct_binding(mod);
    add_ScalarPairWithPadding_binding(mod);
    add_StructArithmetic_binding(mod);
    add_StructWithSlices_binding(mod);
    add_OptionStruct_binding(mod);
    add_Unnamespaced_binding(mod);
    add_CallbackHolder_binding(mod);
    add_MutableCallbackHolder_binding(mod);
    add_Bar_binding(mod);
    add_Foo_binding(mod);
    add_One_binding(mod);
    add_OpaqueThin_binding(mod);
    add_OpaqueThinIter_binding(mod);
    add_OpaqueThinVec_binding(mod);
    add_Two_binding(mod);
    add_OptionOpaque_binding(mod);
    add_OptionOpaqueChar_binding(mod);
    add_OptionString_binding(mod);
    add_ResultOpaque_binding(mod);
    add_RefList_binding(mod);
    add_RefListParameter_binding(mod);
    add_Float64Vec_binding(mod);
    add_Float64VecError_binding(mod);
    add_MyString_binding(mod);
    add_MyOpaqueEnum_binding(mod);
    add_Opaque_binding(mod);
    add_OpaqueMutexedString_binding(mod);
    add_PrimitiveStructVec_binding(mod);
    add_Utf16Wrap_binding(mod);
    add_UnimportedEnum_binding(mod);
    add_OptionEnum_binding(mod);
    add_ErrorEnum_binding(mod);
    add_ContiguousEnum_binding(mod);
    add_DefaultEnum_binding(mod);
    add_MyEnum_binding(mod);
    add_free_function_binding(mod);
    
    nested::ns::add_Nested_binding(nested_ns_mod);
    nested::ns::add_free_function_binding(nested_ns_mod);
    
    nested::ns2::add_Nested_binding(nested_ns2_mod);
    
    ns::add_RenamedDeprecatedStruct_binding(ns_mod);
    ns::add_RenamedStructWithAttrs_binding(ns_mod);
    ns::add_RenamedTestMacroStruct_binding(ns_mod);
    ns::add_AttrOpaque1Renamed_binding(ns_mod);
    ns::add_RenamedAttrOpaque2_binding(ns_mod);
    ns::add_RenamedBlockOverride_binding(ns_mod);
    ns::add_RenamedComparable_binding(ns_mod);
    ns::add_RenamedDeprecatedOpaque_binding(ns_mod);
    ns::add_RenamedMyIndexer_binding(ns_mod);
    ns::add_RenamedMyIterable_binding(ns_mod);
    ns::add_RenamedMyIterator_binding(ns_mod);
    ns::add_RenamedOpaqueArithmetic_binding(ns_mod);
    ns::add_RenamedOpaqueIterable_binding(ns_mod);
    ns::add_RenamedOpaqueIterator_binding(ns_mod);
    ns::add_RenamedOpaqueRefIterable_binding(ns_mod);
    ns::add_RenamedOpaqueRefIterator_binding(ns_mod);
    ns::add_RenamedStringList_binding(ns_mod);
    ns::add_RenamedTestOpaque_binding(ns_mod);
    ns::add_RenamedVectorTest_binding(ns_mod);
    ns::add_RenamedAttrEnum_binding(ns_mod);
    ns::add_RenamedDeprecatedEnum_binding(ns_mod);
    ns::add_free_function_binding(ns_mod);
    
    
}