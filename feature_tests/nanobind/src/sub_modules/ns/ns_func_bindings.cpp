#include "diplomat_nanobind_common.hpp"


#include "ns/free_ns_functions.hpp"


namespace ns{

void add_ns_func_binding(nb::handle mod) {
    mod
    	.def("Renamedfree_func_test", &Renamedfree_func_test, "x"_a);
}


}
