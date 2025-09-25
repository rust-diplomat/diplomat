#include "diplomat_nanobind_common.hpp"


#include "ResultOpaque.hpp"


void add_ResultOpaque_binding(nb::module_ mod) {
    PyType_Slot ResultOpaque_slots[] = {
        {Py_tp_free, (void *)ResultOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ResultOpaque>(mod, "ResultOpaque", nb::type_slots(ResultOpaque_slots))
        .def("assert_integer", &ResultOpaque::assert_integer, "i"_a)
        .def(nb::new_(&ResultOpaque::new_), "i"_a)
        .def_static("new_failing_bar", &ResultOpaque::new_failing_bar)
        .def_static("new_failing_foo", &ResultOpaque::new_failing_foo)
        .def_static("new_failing_struct", &ResultOpaque::new_failing_struct, "i"_a)
        .def_static("new_failing_unit", &ResultOpaque::new_failing_unit)
        .def_static("new_in_enum_err", &ResultOpaque::new_in_enum_err, "i"_a)
        .def_static("new_in_err", &ResultOpaque::new_in_err, "i"_a)
        .def_static("new_int", &ResultOpaque::new_int, "i"_a)
        .def("takes_str", &ResultOpaque::takes_str, "_v"_a, nb::rv_policy::reference_internal);
}

