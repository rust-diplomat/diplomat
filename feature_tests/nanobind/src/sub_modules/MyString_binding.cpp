#include "diplomat_nanobind_common.hpp"


#include "MyString.hpp"


void add_MyString_binding(nb::module_ mod) {
    PyType_Slot MyString_slots[] = {
        {Py_tp_free, (void *)MyString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<MyString>(mod, "MyString", nb::type_slots(MyString_slots))
        .def("borrow", &MyString::borrow)
        .def_static("get_static_str", &MyString::get_static_str)
        .def(nb::new_(&MyString::new_), "v"_a)
        .def_static("new_from_first", &MyString::new_from_first, "v"_a)
        .def_static("new_owned", &MyString::new_owned, "v"_a)
        .def_static("new_unsafe", &MyString::new_unsafe, "v"_a ) // unsupported special method NamedConstructor(Some("unsafe"))
        .def_prop_rw("str", &MyString::get_str, &MyString::set_str)
        .def_static("string_transform", &MyString::string_transform, "foo"_a);
}

