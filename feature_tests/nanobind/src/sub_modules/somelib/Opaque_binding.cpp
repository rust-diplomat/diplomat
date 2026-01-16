#include "diplomat_nanobind_common.hpp"


#include "MyStruct.hpp"
#include "Opaque.hpp"

namespace somelib {
void add_Opaque_binding(nb::module_ mod) {
    PyType_Slot somelib_Opaque_slots[] = {
        {Py_tp_free, (void *)somelib::Opaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Opaque> opaque(mod, "Opaque", nb::type_slots(somelib_Opaque_slots));
    opaque
        .def("assert_struct", &somelib::Opaque::assert_struct, "s"_a)
        .def_static("cmp", &somelib::Opaque::cmp)
        .def_static("from_str", &somelib::Opaque::from_str, "input"_a)
        .def("get_debug_str", &somelib::Opaque::get_debug_str)
        .def(nb::new_(&somelib::Opaque::new_))
        .def_static("returns_imported", &somelib::Opaque::returns_imported)
        .def_static("returns_usize", &somelib::Opaque::returns_usize)
        .def_static("try_from_utf8", &somelib::Opaque::try_from_utf8, "input"_a);
}

} 