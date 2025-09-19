#include "diplomat_nanobind_common.hpp"


#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"


void add_OptionInputStruct_binding(nb::module_ mod) {
    nb::class_<OptionInputStruct>(mod, "OptionInputStruct")
        .def(nb::init<>())
        .def(nb::init<std::optional<uint8_t>, std::optional<char32_t>, std::optional<OptionEnum>>(), "a"_a.none(),  "b"_a.none(),  "c"_a.none())
        .def_rw("a", &OptionInputStruct::a)
        .def_rw("b", &OptionInputStruct::b)
        .def_rw("c", &OptionInputStruct::c);
}

