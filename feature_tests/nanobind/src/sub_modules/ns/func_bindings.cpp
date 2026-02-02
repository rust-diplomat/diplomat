#include "diplomat_nanobind_common.hpp"


#include "ns/free_functions.hpp"

namespace somelib::ns {
void add_free_function_binding(nb::module_ mod) {
    mod
    
        .def("Renamedfree_func_test", &Renamedfree_func_test, "x"_a);
}

} 