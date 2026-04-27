#include "diplomat_nanobind_common.hpp"


#include "ContainingTuple.hpp"
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "TupleStruct.hpp"

namespace somelib {
void add_TupleStruct_binding(nb::module_ mod) {
    nb::class_<somelib::TupleStruct> st(mod, "TupleStruct");
    st
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t, somelib::MyStruct, somelib::Opaque*>(), "x"_a.none(),  "y"_a.none(),  "st"_a.none(),  "op"_a.none())
        .def_rw("x", &somelib::TupleStruct::x)
        .def_rw("y", &somelib::TupleStruct::y)
        .def_rw("st", &somelib::TupleStruct::st)
        .def_rw("op", &somelib::TupleStruct::op)
        .def_static("takes_containing", &somelib::TupleStruct::takes_containing, "c"_a)
        .def_static("takes_st_as_tuple", &somelib::TupleStruct::takes_st_as_tuple, "a"_a);
}

} 