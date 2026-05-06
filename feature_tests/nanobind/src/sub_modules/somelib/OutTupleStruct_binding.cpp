#include "diplomat_nanobind_common.hpp"


#include "Opaque.hpp"
#include "OutTupleStruct.hpp"
#include "PrimitiveStruct.hpp"

namespace somelib {
void add_OutTupleStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OutTupleStruct> st(mod, "OutTupleStruct");
    st
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t, somelib::PrimitiveStruct, std::unique_ptr<somelib::Opaque>>(), "x"_a.none(),  "y"_a.none(),  "primitive"_a.none(),  "opaque"_a)
        .def_rw("x", &somelib::OutTupleStruct::x)
        .def_rw("y", &somelib::OutTupleStruct::y)
        .def_rw("primitive", &somelib::OutTupleStruct::primitive)
        .def_prop_rw("opaque", 
            [](const somelib::OutTupleStruct& self) { return self.opaque.get(); },
            [](somelib::OutTupleStruct& self, std::unique_ptr<somelib::Opaque>&& v) { self.opaque = std::move(v); }
        )
        .def_static("new", &somelib::OutTupleStruct::new_);
}

} 