#include "diplomat_nanobind_common.hpp"


#include "MyZst.hpp"

namespace somelib {
void add_MyZst_binding(nb::module_ mod) {
    nb::class_<somelib::MyZst> st(mod, "MyZst");
    st
        .def(nb::init<>());
}

} 