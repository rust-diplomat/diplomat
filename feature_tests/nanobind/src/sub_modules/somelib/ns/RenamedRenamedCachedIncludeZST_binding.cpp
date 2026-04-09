#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedRenamedCachedIncludeZST.hpp"

namespace somelib::ns {
void add_RenamedRenamedCachedIncludeZST_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedRenamedCachedIncludeZST> st(mod, "RenamedRenamedCachedIncludeZST");
    st
        .def(nb::init<>());
}

} 