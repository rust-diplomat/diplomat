#include "diplomat_nanobind_common.hpp"


#include "ResultOpaque.hpp"

namespace somelib {
void add_ResultOpaque_binding(nb::module_ mod) {
    PyType_Slot somelib_ResultOpaque_slots[] = {
        {Py_tp_free, (void *)somelib::ResultOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ResultOpaque> opaque(mod, "ResultOpaque", nb::type_slots(somelib_ResultOpaque_slots));
    opaque
        .def("assert_integer", &somelib::ResultOpaque::assert_integer, "i"_a)
        .def(nb::new_(&somelib::ResultOpaque::new_), "i"_a)
        .def_static("new_failing_bar", &somelib::ResultOpaque::new_failing_bar)
        .def_static("new_failing_foo", &somelib::ResultOpaque::new_failing_foo)
        .def_static("new_failing_struct", &somelib::ResultOpaque::new_failing_struct, "i"_a)
        .def_static("new_failing_unit", &somelib::ResultOpaque::new_failing_unit)
        .def_static("new_in_enum_err", &somelib::ResultOpaque::new_in_enum_err, "i"_a)
        .def_static("new_in_err", &somelib::ResultOpaque::new_in_err, "i"_a)
        .def_static("new_int", &somelib::ResultOpaque::new_int, "i"_a)
        .def("takes_str", &somelib::ResultOpaque::takes_str, "_v"_a, nb::rv_policy::reference_internal);
}

} 