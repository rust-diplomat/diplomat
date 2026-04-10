#include "diplomat_nanobind_common.hpp"


#include "mylib/Decimal.hpp"

namespace somelib::mylib {
void add_Decimal_binding(nb::module_ mod) {
    PyType_Slot somelib_mylib_Decimal_slots[] = {
        {Py_tp_free, (void *)somelib::mylib::Decimal::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::mylib::Decimal> opaque(mod, "Decimal", nb::type_slots(somelib_mylib_Decimal_slots));
    opaque
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int32_t>(&somelib::mylib::Decimal::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int64_t>(&somelib::mylib::Decimal::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<uint32_t>(&somelib::mylib::Decimal::from))), "_v"_a);
}

} 