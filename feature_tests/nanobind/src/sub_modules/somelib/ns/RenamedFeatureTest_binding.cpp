#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedFeatureTest.hpp"

namespace somelib::ns {
void add_RenamedFeatureTest_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedFeatureTest> st(mod, "RenamedFeatureTest");
    st
        .def(nb::init<>());
}

} 