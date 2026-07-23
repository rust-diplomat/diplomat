#include "diplomat_nanobind_common.hpp"


#include "Float64Vec.hpp"
#include "MyString.hpp"

namespace somelib {
void add_MyString_binding(nb::module_ mod) {
    nb::class_<somelib::MyString> opaque(mod, "MyString");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::MyString::new_))), "v"_a='T')
        .def("borrow", &somelib::MyString::borrow)
        .def_static("get_static_str", &somelib::MyString::get_static_str)
        .def_static("new_from_first", std::move(maybe_op_unwrap(&somelib::MyString::new_from_first)), "v"_a)
        .def_static("new_from_utf16", std::move(maybe_op_unwrap(&somelib::MyString::new_from_utf16)), "v"_a)
        .def_static("new_unsafe", std::move(maybe_op_unwrap(&somelib::MyString::new_unsafe)), "v"_a)
        .def_static("optional_slice_of_opaques", &somelib::MyString::optional_slice_of_opaques, "sl"_a)
        .def_static("other_opaque_type", &somelib::MyString::other_opaque_type, "other"_a)
        .def_static("slice_of_opaques", &somelib::MyString::slice_of_opaques, "sl"_a)
        .def_prop_rw("str", &somelib::MyString::get_str, &somelib::MyString::set_str)
        .def_static("string_transform", &somelib::MyString::string_transform, "foo"_a);
}

} 