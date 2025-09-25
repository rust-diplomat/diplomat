#include "diplomat_nanobind_common.hpp"


#include "free_functions.hpp"


void add_diplomat_func_binding(nb::module_ mod) {
    mod
        .def("free_callback_holder", &free_callback_holder, "f"_a);
}

