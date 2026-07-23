#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedComparable.hpp"

namespace somelib::ns {
void add_RenamedComparable_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedComparable> opaque(mod, "RenamedComparable");
    opaque
        .def(nb::self == nb::self)
            .def(nb::self != nb::self)
            .def(nb::self <= nb::self)
            .def(nb::self >= nb::self)
            .def(nb::self < nb::self)
            .def(nb::self > nb::self)
        .def_static("new", std::move(maybe_op_unwrap(&somelib::ns::RenamedComparable::new_)), "int"_a);
}

} 