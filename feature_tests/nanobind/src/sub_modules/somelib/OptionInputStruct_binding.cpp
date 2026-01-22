#include "diplomat_nanobind_common.hpp"


#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"

namespace somelib {
void add_OptionInputStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OptionInputStruct> st(mod, "OptionInputStruct");
    st
        .def(nb::init<>())
        .def(nb::init<std::optional<uint8_t>, std::optional<char32_t>, std::optional<somelib::OptionEnum>>(), "a"_a.none(),  "b"_a.none(),  "c"_a.none())
        .def_rw("a", &somelib::OptionInputStruct::a)
        .def_rw("b", &somelib::OptionInputStruct::b)
        .def_rw("c", &somelib::OptionInputStruct::c);
}

} 