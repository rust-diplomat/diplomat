#include "diplomat_nanobind_common.hpp"


#include "ImportedStruct.hpp"
#include "UnimportedEnum.hpp"

namespace somelib {
void add_ImportedStruct_binding(nb::module_ mod) {
    nb::class_<somelib::ImportedStruct> st(mod, "ImportedStruct");
    st
        .def(nb::init<>())
        .def(nb::init<somelib::UnimportedEnum, uint8_t>(), "foo"_a.none(),  "count"_a.none())
        .def_rw("foo", &somelib::ImportedStruct::foo)
        .def_rw("count", &somelib::ImportedStruct::count);
}

} 