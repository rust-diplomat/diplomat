#include "diplomat_nanobind_common.hpp"


#include "PrimitiveStruct.hpp"


void add_PrimitiveStruct_binding(nb::handle mod) {
    
    NB_MAKE_OPAQUE(std::vector<PrimitiveStruct>)
    nb::bind_vector<std::vector<PrimitiveStruct>>(mod, "PrimitiveStructSlice"); 
    nb::class_<PrimitiveStruct>(mod, "PrimitiveStruct")
        .def(nb::init<>())
        .def(nb::init<float, bool, char32_t, int64_t, intptr_t, uint8_t>(), "x"_a.none(),  "a"_a.none(),  "b"_a.none(),  "c"_a.none(),  "d"_a.none(),  "e"_a.none())
        .def_rw("x", &PrimitiveStruct::x)
        .def_rw("a", &PrimitiveStruct::a)
        .def_rw("b", &PrimitiveStruct::b)
        .def_rw("c", &PrimitiveStruct::c)
        .def_rw("d", &PrimitiveStruct::d)
        .def_rw("e", &PrimitiveStruct::e)
    	.def_static("mutable_slice", &PrimitiveStruct::mutable_slice, "a"_a);
}

