#include "diplomat_nanobind_common.hpp"


#include "nested/ns/free_functions.hpp"

namespace somelib::nested::ns {
void add_free_function_binding(nb::module_ mod) {
    mod
    
        .def("Renamednested_ns_fn", &Renamednested_ns_fn, "x"_a);
}

} 