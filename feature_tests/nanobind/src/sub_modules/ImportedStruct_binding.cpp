#include "diplomat_nanobind_common.hpp"


#include "ImportedStruct.hpp"
#include "UnimportedEnum.hpp"


void add_ImportedStruct_binding(nb::module_ mod) {
    nb::class_<ImportedStruct>(mod, "ImportedStruct")
        .def(nb::init<>())
        .def(nb::init<UnimportedEnum, uint8_t>(), "foo"_a.none(),  "count"_a.none())
        .def_rw("foo", &ImportedStruct::foo)
        .def_rw("count", &ImportedStruct::count);
}

