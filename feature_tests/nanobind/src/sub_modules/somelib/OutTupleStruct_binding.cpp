#include "diplomat_nanobind_common.hpp"


#include "Opaque.hpp"
#include "OutTupleStruct.hpp"
#include "PrimitiveStruct.hpp"

namespace somelib {
void add_OutTupleStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OutTupleStruct> st(mod, "OutTupleStruct");
    st
        .def_ro("x", &somelib::OutTupleStruct::x)
        .def_ro("y", &somelib::OutTupleStruct::y)
        .def_ro("primitive", &somelib::OutTupleStruct::primitive)
        .def_ro("opaque", &somelib::OutTupleStruct::opaque)
        .def_static("new", &somelib::OutTupleStruct::new_);
}

} 