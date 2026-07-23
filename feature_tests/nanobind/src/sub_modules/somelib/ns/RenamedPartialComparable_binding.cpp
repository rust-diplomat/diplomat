#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedPartialComparable.hpp"

namespace somelib::ns {
void add_RenamedPartialComparable_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedPartialComparable> opaque(mod, "RenamedPartialComparable");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedPartialComparable::new_))), "float_"_a)
        .def(nb::self == nb::self)
            .def(nb::self != nb::self)
            .def(nb::self <= nb::self)
            .def(nb::self >= nb::self)
            .def(nb::self < nb::self)
            .def(nb::self > nb::self)
        .def("test_nonstd", &somelib::ns::RenamedPartialComparable::test_nonstd, "other"_a);
}

} 