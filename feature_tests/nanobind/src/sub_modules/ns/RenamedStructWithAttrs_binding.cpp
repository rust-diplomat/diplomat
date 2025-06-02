#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedStructWithAttrs.hpp"


namespace ns{

void add_RenamedStructWithAttrs_binding(nb::handle mod) {
    nb::class_<ns::RenamedStructWithAttrs>(mod, "RenamedStructWithAttrs")
        .def_rw("a", &ns::RenamedStructWithAttrs::a)
        .def_rw("b", &ns::RenamedStructWithAttrs::b)
    	.def_prop_ro("c", &ns::RenamedStructWithAttrs::c)
    	.def("__init__", [](ns::RenamedStructWithAttrs* self, bool a, uint32_t b){ auto tmp = ns::RenamedStructWithAttrs::new_fallible(a, b);
    				if(tmp.is_ok()) {
    					*self = std::move(tmp).ok().value();
    				} else {
    					nb::cast(tmp); // This will raise a python error with the contents of the error type
    				}}, "a"_a, "b"_a);
}


}
