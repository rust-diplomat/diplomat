#include "diplomat_nanobind_common.hpp"


#include "mylib/MethodOverloading.hpp"

namespace somelib::mylib {
void add_MethodOverloading_binding(nb::module_ mod) {
    PyType_Slot somelib_mylib_MethodOverloading_slots[] = {
        {Py_tp_free, (void *)somelib::mylib::MethodOverloading::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::mylib::MethodOverloading> opaque(mod, "MethodOverloading", nb::type_slots(somelib_mylib_MethodOverloading_slots));
    opaque
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int32_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int64_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<uint32_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a);
}

} 