#include "diplomat_nanobind_common.hpp"


#include "OpaqueMutexedString.hpp"

namespace somelib {
void add_OpaqueMutexedString_binding(nb::module_ mod) {
    nb::class_<somelib::OpaqueMutexedString> opaque(mod, "OpaqueMutexedString");
    opaque
        .def("borrow", &somelib::OpaqueMutexedString::borrow, nb::rv_policy::reference_internal)
        .def_static("borrow_other", &somelib::OpaqueMutexedString::borrow_other, "other"_a, nb::rv_policy::reference)
        .def("borrow_self_or_other", &somelib::OpaqueMutexedString::borrow_self_or_other, "other"_a, nb::rv_policy::reference_internal)
        .def("change", &somelib::OpaqueMutexedString::change, "number"_a)
        .def("dummy_str", &somelib::OpaqueMutexedString::dummy_str)
        .def_static("from_usize", std::move(maybe_op_unwrap(&somelib::OpaqueMutexedString::from_usize)), "number"_a)
        .def("get_len_and_add", &somelib::OpaqueMutexedString::get_len_and_add, "other"_a)
        .def("to_unsigned_from_unsigned", &somelib::OpaqueMutexedString::to_unsigned_from_unsigned, "input"_a)
        .def("wrapper", std::move(maybe_op_unwrap(&somelib::OpaqueMutexedString::wrapper)));
}

} 