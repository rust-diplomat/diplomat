#include "diplomat_nanobind_common.hpp"


#include "MyStruct.hpp"
#include "Opaque.hpp"


void add_Opaque_binding(nb::module_ mod) {
    PyType_Slot Opaque_slots[] = {
        {Py_tp_free, (void *)Opaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Opaque>(mod, "Opaque", nb::type_slots(Opaque_slots))
        .def("assert_struct", &Opaque::assert_struct, "s"_a)
        .def_static("cmp", &Opaque::cmp)
        .def_static("from_str", &Opaque::from_str, "input"_a)
        .def("get_debug_str", &Opaque::get_debug_str)
        .def(nb::new_(&Opaque::new_))
        .def_static("returns_imported", &Opaque::returns_imported)
        .def_static("returns_usize", &Opaque::returns_usize)
        .def_static("try_from_utf8", &Opaque::try_from_utf8, "input"_a);
}

