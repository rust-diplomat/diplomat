#include "diplomat_nanobind_common.hpp"


#include "OptionString.hpp"

namespace somelib {
void add_OptionString_binding(nb::module_ mod) {
    nb::class_<somelib::OptionString> opaque(mod, "OptionString");
    opaque
        .def("borrow", &somelib::OptionString::borrow)
        .def_static("new", std::move(maybe_op_unwrap(&somelib::OptionString::new_)), "diplomat_str"_a)
        .def("write", &somelib::OptionString::write);
}

} 