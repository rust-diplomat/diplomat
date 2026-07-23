#include "diplomat_nanobind_common.hpp"


#include "Utf16Wrap.hpp"

namespace somelib {
void add_Utf16Wrap_binding(nb::module_ mod) {
    nb::class_<somelib::Utf16Wrap> opaque(mod, "Utf16Wrap");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::Utf16Wrap::from_utf16))), "input"_a)
        .def("borrow_cont", &somelib::Utf16Wrap::borrow_cont)
        .def("get_debug_str", &somelib::Utf16Wrap::get_debug_str);
}

} 