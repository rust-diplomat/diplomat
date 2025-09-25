#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedStructWithAttrs.hpp"
NB_MAKE_OPAQUE(std::vector<ns::RenamedStructWithAttrs>)


namespace ns{

void add_RenamedStructWithAttrs_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<ns::RenamedStructWithAttrs> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<ns::RenamedStructWithAttrs>>(mod, "ns::RenamedStructWithAttrsSlice"); 
    nb::class_<ns::RenamedStructWithAttrs>(mod, "RenamedStructWithAttrs")
        .def_rw("a", &ns::RenamedStructWithAttrs::a)
        .def_rw("b", &ns::RenamedStructWithAttrs::b)
        .def_prop_ro("c", &ns::RenamedStructWithAttrs::c)
        .def("deprecated", &ns::RenamedStructWithAttrs::deprecated)
        .def("__init__", [](ns::RenamedStructWithAttrs* self, bool a, uint32_t b){ auto tmp = ns::RenamedStructWithAttrs::new_fallible(a, b);
                    if(tmp.is_ok()) {
                        *self = std::move(tmp).ok().value();
                    } else {
                        nb::cast(tmp); // This will raise a python error with the contents of the error type
                    }}, "a"_a, "b"_a);
}


}
