#include <nanobind/nanobind.h>
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "CallbackTestingStruct.hpp"
#include "CallbackWrapper.hpp"
#include "ContiguousEnum.hpp"
#include "CyclicStructA.hpp"
#include "CyclicStructB.hpp"
#include "CyclicStructC.hpp"
#include "DefaultEnum.hpp"
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"
#include "Float64Vec.hpp"
#include "Foo.hpp"
#include "ImportedStruct.hpp"
#include "MyEnum.hpp"
#include "MyOpaqueEnum.hpp"
#include "MyString.hpp"
#include "MyStruct.hpp"
#include "MyZst.hpp"
#include "NestedBorrowedFields.hpp"
#include "One.hpp"
#include "Opaque.hpp"
#include "OpaqueMutexedString.hpp"
#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionString.hpp"
#include "OptionStruct.hpp"
#include "RefList.hpp"
#include "RefListParameter.hpp"
#include "ResultOpaque.hpp"
#include "Two.hpp"
#include "UnimportedEnum.hpp"
#include "Unnamespaced.hpp"
#include "Utf16Wrap.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"
#include "ns/RenamedAttrOpaque2.hpp"
#include <nanobind/stl/string_view.h>

namespace nb = nanobind;
using namespace nb::literals;

NB_MODULE(ns, ns)
{
    
    nb::module_ default_root_mod();
    nb::class_<CallbackTestingStruct>(default_root_mod, "CallbackTestingStruct")
        .def_ro("x", &CallbackTestingStruct::x)
        .def_ro("y", &CallbackTestingStruct::y);
    nb::class_<CallbackWrapper>(default_root_mod, "CallbackWrapper")
        .def_ro("cant_be_empty", &CallbackWrapper::cant_be_empty)
    	.def_static("test_multi_arg_callback", &CallbackWrapper::test_multi_arg_callback, "f"_a, "x"_a)
    	.def_static("test_no_args", &CallbackWrapper::test_no_args, "h"_a)
    	.def_static("test_cb_with_struct", &CallbackWrapper::test_cb_with_struct, "f"_a)
    	.def_static("test_multiple_cb_args", &CallbackWrapper::test_multiple_cb_args, "f"_a, "g"_a)
    	.def_static("test_str_cb_arg", &CallbackWrapper::test_str_cb_arg, "f"_a);
    nb::class_<ImportedStruct>(default_root_mod, "ImportedStruct")
        .def_ro("foo", &ImportedStruct::foo)
        .def_ro("count", &ImportedStruct::count);
    nb::class_<BorrowedFields>(default_root_mod, "BorrowedFields")
        .def_ro("a", &BorrowedFields::a)
        .def_ro("b", &BorrowedFields::b)
        .def_ro("c", &BorrowedFields::c)
    	.def_static("from_bar_and_strings", &BorrowedFields::from_bar_and_strings, "bar"_a, "dstr16"_a, "utf8_str"_a);
    nb::class_<BorrowedFieldsReturning>(default_root_mod, "BorrowedFieldsReturning")
        .def_ro("bytes", &BorrowedFieldsReturning::bytes);
    nb::class_<BorrowedFieldsWithBounds>(default_root_mod, "BorrowedFieldsWithBounds")
        .def_ro("field_a", &BorrowedFieldsWithBounds::field_a)
        .def_ro("field_b", &BorrowedFieldsWithBounds::field_b)
        .def_ro("field_c", &BorrowedFieldsWithBounds::field_c)
    	.def_static("from_foo_and_strings", &BorrowedFieldsWithBounds::from_foo_and_strings, "foo"_a, "dstr16_x"_a, "utf8_str_z"_a);
    nb::class_<NestedBorrowedFields>(default_root_mod, "NestedBorrowedFields")
        .def_ro("fields", &NestedBorrowedFields::fields)
        .def_ro("bounds", &NestedBorrowedFields::bounds)
        .def_ro("bounds2", &NestedBorrowedFields::bounds2)
    	.def_static("from_bar_and_foo_and_strings", &NestedBorrowedFields::from_bar_and_foo_and_strings, "bar"_a, "foo"_a, "dstr16_x"_a, "dstr16_z"_a, "utf8_str_y"_a, "utf8_str_z"_a);
    nb::class_<OptionInputStruct>(default_root_mod, "OptionInputStruct")
        .def_ro("a", &OptionInputStruct::a)
        .def_ro("b", &OptionInputStruct::b)
        .def_ro("c", &OptionInputStruct::c);
    nb::class_<ErrorStruct>(default_root_mod, "ErrorStruct")
        .def_ro("i", &ErrorStruct::i)
        .def_ro("j", &ErrorStruct::j);
    nb::class_<CyclicStructA>(default_root_mod, "CyclicStructA")
        .def_ro("a", &CyclicStructA::a)
    	.def_static("get_b", &CyclicStructA::get_b)
    	.def("cyclic_out", &CyclicStructA::cyclic_out)
    	.def("double_cyclic_out", &CyclicStructA::double_cyclic_out, "cyclic_struct_a"_a)
    	.def("getter_out", &CyclicStructA::getter_out);
    nb::class_<CyclicStructB>(default_root_mod, "CyclicStructB")
        .def_ro("field", &CyclicStructB::field)
    	.def_static("get_a", &CyclicStructB::get_a)
    	.def_static("get_a_option", &CyclicStructB::get_a_option);
    nb::class_<CyclicStructC>(default_root_mod, "CyclicStructC")
        .def_ro("a", &CyclicStructC::a)
    	.def_static("takes_nested_parameters", &CyclicStructC::takes_nested_parameters, "c"_a)
    	.def("cyclic_out", &CyclicStructC::cyclic_out);
    nb::class_<MyStruct>(default_root_mod, "MyStruct")
        .def_ro("a", &MyStruct::a)
        .def_ro("b", &MyStruct::b)
        .def_ro("c", &MyStruct::c)
        .def_ro("d", &MyStruct::d)
        .def_ro("e", &MyStruct::e)
        .def_ro("f", &MyStruct::f)
        .def_ro("g", &MyStruct::g)
    	.def_static("new_", &MyStruct::new_)
    	.def("into_a", &MyStruct::into_a)
    	.def_static("returns_zst_result", &MyStruct::returns_zst_result)
    	.def_static("fails_zst_result", &MyStruct::fails_zst_result);
    nb::class_<MyZst>(default_root_mod, "MyZst");
    nb::class_<OptionStruct>(default_root_mod, "OptionStruct")
        .def_ro("a", &OptionStruct::a)
        .def_ro("b", &OptionStruct::b)
        .def_ro("c", &OptionStruct::c)
        .def_ro("d", &OptionStruct::d);
    nb::module_ ns_mod();
    nb::class_<ns::AttrOpaque1Renamed>(ns_mod, "AttrOpaque1Renamed")
    	.def_static("totally_not_new", &ns::AttrOpaque1Renamed::totally_not_new)
    	.def("method_renamed", &ns::AttrOpaque1Renamed::method_renamed)
    	.def("abirenamed", &ns::AttrOpaque1Renamed::abirenamed)
    	.def("use_unnamespaced", &ns::AttrOpaque1Renamed::use_unnamespaced, "_un"_a)
    	.def("use_namespaced", &ns::AttrOpaque1Renamed::use_namespaced, "_n"_a);
    nb::class_<ns::RenamedAttrOpaque2>(ns_mod, "RenamedAttrOpaque2");
    nb::class_<Unnamespaced>(default_root_mod, "Unnamespaced")
    	.def_static("make", &Unnamespaced::make, "_e"_a)
    	.def("use_namespaced", &Unnamespaced::use_namespaced, "_n"_a);
    nb::class_<Bar>(default_root_mod, "Bar")
    	.def("foo", &Bar::foo);
    nb::class_<Foo>(default_root_mod, "Foo")
    	.def_static("new_", &Foo::new_, "x"_a)
    	.def("get_bar", &Foo::get_bar)
    	.def_static("new_static", &Foo::new_static, "x"_a)
    	.def("as_returning", &Foo::as_returning)
    	.def_static("extract_from_fields", &Foo::extract_from_fields, "fields"_a)
    	.def_static("extract_from_bounds", &Foo::extract_from_bounds, "bounds"_a, "another_string"_a);
    nb::class_<One>(default_root_mod, "One")
    	.def_static("transitivity", &One::transitivity, "hold"_a, "nohold"_a)
    	.def_static("cycle", &One::cycle, "hold"_a, "nohold"_a)
    	.def_static("many_dependents", &One::many_dependents, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a)
    	.def_static("return_outlives_param", &One::return_outlives_param, "hold"_a, "nohold"_a)
    	.def_static("diamond_top", &One::diamond_top, "top"_a, "left"_a, "right"_a, "bottom"_a)
    	.def_static("diamond_left", &One::diamond_left, "top"_a, "left"_a, "right"_a, "bottom"_a)
    	.def_static("diamond_right", &One::diamond_right, "top"_a, "left"_a, "right"_a, "bottom"_a)
    	.def_static("diamond_bottom", &One::diamond_bottom, "top"_a, "left"_a, "right"_a, "bottom"_a)
    	.def_static("diamond_and_nested_types", &One::diamond_and_nested_types, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a)
    	.def_static("implicit_bounds", &One::implicit_bounds, "explicit_hold"_a, "implicit_hold"_a, "nohold"_a)
    	.def_static("implicit_bounds_deep", &One::implicit_bounds_deep, "explicit_"_a, "implicit_1"_a, "implicit_2"_a, "nohold"_a);
    nb::class_<Two>(default_root_mod, "Two");
    nb::class_<OptionOpaque>(default_root_mod, "OptionOpaque")
    	.def_static("new_", &OptionOpaque::new_, "i"_a)
    	.def_static("new_none", &OptionOpaque::new_none)
    	.def_static("returns", &OptionOpaque::returns)
    	.def("option_isize", &OptionOpaque::option_isize)
    	.def("option_usize", &OptionOpaque::option_usize)
    	.def("option_i32", &OptionOpaque::option_i32)
    	.def("option_u32", &OptionOpaque::option_u32)
    	.def_static("new_struct", &OptionOpaque::new_struct)
    	.def_static("new_struct_nones", &OptionOpaque::new_struct_nones)
    	.def("assert_integer", &OptionOpaque::assert_integer, "i"_a)
    	.def_static("option_opaque_argument", &OptionOpaque::option_opaque_argument, "arg"_a)
    	.def_static("accepts_option_u8", &OptionOpaque::accepts_option_u8, "arg"_a)
    	.def_static("accepts_option_enum", &OptionOpaque::accepts_option_enum, "arg"_a)
    	.def_static("accepts_option_input_struct", &OptionOpaque::accepts_option_input_struct, "arg"_a)
    	.def_static("returns_option_input_struct", &OptionOpaque::returns_option_input_struct);
    nb::class_<OptionOpaqueChar>(default_root_mod, "OptionOpaqueChar")
    	.def("assert_char", &OptionOpaqueChar::assert_char, "ch"_a);
    nb::class_<OptionString>(default_root_mod, "OptionString")
    	.def_static("new_", &OptionString::new_, "diplomat_str"_a)
    	.def("write", &OptionString::write)
    	.def("borrow", &OptionString::borrow);
    nb::class_<ResultOpaque>(default_root_mod, "ResultOpaque")
    	.def_static("new_", &ResultOpaque::new_, "i"_a)
    	.def_static("new_failing_foo", &ResultOpaque::new_failing_foo)
    	.def_static("new_failing_bar", &ResultOpaque::new_failing_bar)
    	.def_static("new_failing_unit", &ResultOpaque::new_failing_unit)
    	.def_static("new_failing_struct", &ResultOpaque::new_failing_struct, "i"_a)
    	.def_static("new_in_err", &ResultOpaque::new_in_err, "i"_a)
    	.def_static("new_int", &ResultOpaque::new_int, "i"_a)
    	.def_static("new_in_enum_err", &ResultOpaque::new_in_enum_err, "i"_a)
    	.def("assert_integer", &ResultOpaque::assert_integer, "i"_a);
    nb::class_<RefList>(default_root_mod, "RefList")
    	.def_static("node", &RefList::node, "data"_a);
    nb::class_<RefListParameter>(default_root_mod, "RefListParameter");
    nb::class_<Float64Vec>(default_root_mod, "Float64Vec")
    	.def_static("new_", &Float64Vec::new_, "v"_a)
    	.def_static("new_bool", &Float64Vec::new_bool, "v"_a)
    	.def_static("new_i16", &Float64Vec::new_i16, "v"_a)
    	.def_static("new_u16", &Float64Vec::new_u16, "v"_a)
    	.def_static("new_isize", &Float64Vec::new_isize, "v"_a)
    	.def_static("new_usize", &Float64Vec::new_usize, "v"_a)
    	.def_static("new_f64_be_bytes", &Float64Vec::new_f64_be_bytes, "v"_a)
    	.def("as_slice", &Float64Vec::as_slice)
    	.def("fill_slice", &Float64Vec::fill_slice, "v"_a)
    	.def("set_value", &Float64Vec::set_value, "new_slice"_a)
    	.def("to_string", &Float64Vec::to_string)
    	.def("borrow", &Float64Vec::borrow)
    	.def("get", &Float64Vec::get, "i"_a);
    nb::class_<MyString>(default_root_mod, "MyString")
    	.def_static("new_", &MyString::new_, "v"_a)
    	.def_static("new_unsafe", &MyString::new_unsafe, "v"_a)
    	.def_static("new_owned", &MyString::new_owned, "v"_a)
    	.def_static("new_from_first", &MyString::new_from_first, "v"_a)
    	.def("set_str", &MyString::set_str, "new_str"_a)
    	.def("get_str", &MyString::get_str)
    	.def_static("string_transform", &MyString::string_transform, "foo"_a)
    	.def("borrow", &MyString::borrow);
    nb::class_<MyOpaqueEnum>(default_root_mod, "MyOpaqueEnum")
    	.def_static("new_", &MyOpaqueEnum::new_)
    	.def("to_string", &MyOpaqueEnum::to_string);
    nb::class_<Opaque>(default_root_mod, "Opaque")
    	.def_static("new_", &Opaque::new_)
    	.def_static("try_from_utf8", &Opaque::try_from_utf8, "input"_a)
    	.def_static("from_str", &Opaque::from_str, "input"_a)
    	.def("get_debug_str", &Opaque::get_debug_str)
    	.def("assert_struct", &Opaque::assert_struct, "s"_a)
    	.def_static("returns_usize", &Opaque::returns_usize)
    	.def_static("returns_imported", &Opaque::returns_imported)
    	.def_static("cmp", &Opaque::cmp);
    nb::class_<OpaqueMutexedString>(default_root_mod, "OpaqueMutexedString")
    	.def_static("from_usize", &OpaqueMutexedString::from_usize, "number"_a)
    	.def("change", &OpaqueMutexedString::change, "number"_a)
    	.def("borrow", &OpaqueMutexedString::borrow)
    	.def_static("borrow_other", &OpaqueMutexedString::borrow_other, "other"_a)
    	.def("borrow_self_or_other", &OpaqueMutexedString::borrow_self_or_other, "other"_a)
    	.def("get_len_and_add", &OpaqueMutexedString::get_len_and_add, "other"_a)
    	.def("dummy_str", &OpaqueMutexedString::dummy_str)
    	.def("wrapper", &OpaqueMutexedString::wrapper)
    	.def("to_unsigned_from_unsigned", &OpaqueMutexedString::to_unsigned_from_unsigned, "input"_a);
    nb::class_<Utf16Wrap>(default_root_mod, "Utf16Wrap")
    	.def_static("from_utf16", &Utf16Wrap::from_utf16, "input"_a)
    	.def("get_debug_str", &Utf16Wrap::get_debug_str)
    	.def("borrow_cont", &Utf16Wrap::borrow_cont);
    nb::enum_<ns::RenamedAttrEnum::Value>(ns_mod, "ns::RenamedAttrEnum")
    	.value("A", ns::RenamedAttrEnum::A)
    	.value("B", ns::RenamedAttrEnum::B)
    	.value("C", ns::RenamedAttrEnum::C);
    	
    nb::enum_<UnimportedEnum::Value>(default_root_mod, "UnimportedEnum")
    	.value("A", UnimportedEnum::A)
    	.value("B", UnimportedEnum::B)
    	.value("C", UnimportedEnum::C);
    	
    nb::enum_<OptionEnum::Value>(default_root_mod, "OptionEnum")
    	.value("Foo", OptionEnum::Foo)
    	.value("Bar", OptionEnum::Bar);
    	
    nb::enum_<ErrorEnum::Value>(default_root_mod, "ErrorEnum")
    	.value("Foo", ErrorEnum::Foo)
    	.value("Bar", ErrorEnum::Bar);
    	
    nb::enum_<ContiguousEnum::Value>(default_root_mod, "ContiguousEnum")
    	.value("C", ContiguousEnum::C)
    	.value("D", ContiguousEnum::D)
    	.value("E", ContiguousEnum::E)
    	.value("F", ContiguousEnum::F);
    	
    nb::enum_<DefaultEnum::Value>(default_root_mod, "DefaultEnum")
    	.value("A", DefaultEnum::A)
    	.value("B", DefaultEnum::B);
    	
    nb::enum_<MyEnum::Value>(default_root_mod, "MyEnum")
    	.value("A", MyEnum::A)
    	.value("B", MyEnum::B)
    	.value("C", MyEnum::C)
    	.value("D", MyEnum::D)
    	.value("E", MyEnum::E)
    	.value("F", MyEnum::F);
    	
}