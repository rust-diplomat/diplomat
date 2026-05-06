#include "diplomat_nanobind_common.hpp"


#include "ContainingTuple.hpp"
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "TupleStruct.hpp"

namespace somelib {
void add_ContainingTuple_binding(nb::module_ mod) {
    nb::class_<somelib::ContainingTuple> st(mod, "ContainingTuple");
    st
        .def(nb::init<>())
        .def(nb::init<std::tuple<int32_t,int32_t,somelib::MyStruct,somelib::Opaque*>>(), "inner"_a.none())
        .def_rw("inner", &somelib::ContainingTuple::inner);
}

} 