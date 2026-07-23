#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedPartialComparableSlice.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::ns::RenamedPartialComparableSlice>)
namespace nanobind::detail { template<> struct is_equality_comparable<somelib::ns::RenamedPartialComparableSlice>{static constexpr bool value = false;}; }

namespace somelib::ns {
void add_RenamedPartialComparableSlice_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::ns::RenamedPartialComparableSlice> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::ns::RenamedPartialComparableSlice>>(mod, "RenamedPartialComparableSliceSlice"); 
    nb::class_<somelib::ns::RenamedPartialComparableSlice> st(mod, "RenamedPartialComparableSlice");
    maybe_bind_default_init(st);
    st
        .def(nb::init<float>(), "f"_a.none())
        .def_rw("f", &somelib::ns::RenamedPartialComparableSlice::f)
        .def(nb::self == nb::self)
            .def(nb::self != nb::self)
            .def(nb::self <= nb::self)
            .def(nb::self >= nb::self)
            .def(nb::self < nb::self)
            .def(nb::self > nb::self);
}

} 