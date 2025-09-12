#include "diplomat_nanobind_common.hpp"


#include "nested/ns/free_nested_ns_functions.hpp"


namespace nested::ns{

void add_nested_ns_func_binding(nb::handle mod) {
    mod
    	.def("Renamednested_ns_fn", &Renamednested_ns_fn, "x"_a);
}


}
