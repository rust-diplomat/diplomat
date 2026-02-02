#include "diplomat_nanobind_common.hpp"


#include "free_functions.hpp"

namespace somelib {
void add_free_function_binding(nb::module_ mod) {
    mod
    
        .def("free_callback_holder", &free_callback_holder, "f"_a);
}

} 