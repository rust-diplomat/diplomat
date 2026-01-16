#include "diplomat_nanobind_common.hpp"


#include "MyString.hpp"

namespace somelib {
void add_MyString_binding(nb::module_ mod) {
    PyType_Slot somelib_MyString_slots[] = {
        {Py_tp_free, (void *)somelib::MyString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::MyString> opaque(mod, "MyString", nb::type_slots(somelib_MyString_slots));
    opaque
        .def("borrow", &somelib::MyString::borrow)
        .def_static("get_static_str", &somelib::MyString::get_static_str)
        .def(nb::new_(&somelib::MyString::new_), "v"_a)
        .def_static("new_from_first", &somelib::MyString::new_from_first, "v"_a)
        .def_static("new_owned", &somelib::MyString::new_owned, "v"_a)
        .def_static("new_unsafe", &somelib::MyString::new_unsafe, "v"_a ) // unsupported special method NamedConstructor(Some("unsafe"))
        .def_prop_rw("str", &somelib::MyString::get_str, &somelib::MyString::set_str)
        .def_static("string_transform", &somelib::MyString::string_transform, "foo"_a);
}

} 