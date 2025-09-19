#include "diplomat_nanobind_common.hpp"


#include "MyZst.hpp"


void add_MyZst_binding(nb::module_ mod) {
    nb::class_<MyZst>(mod, "MyZst")
        .def(nb::init<>());
}

