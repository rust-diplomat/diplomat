#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedStruct.hpp"

namespace somelib::ns {
void add_RenamedDeprecatedStruct_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedDeprecatedStruct>(mod, "RenamedDeprecatedStruct")
        .def(nb::init<>());
}

} 