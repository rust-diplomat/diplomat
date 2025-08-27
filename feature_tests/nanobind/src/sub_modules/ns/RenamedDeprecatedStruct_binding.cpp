#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedStruct.hpp"


namespace ns{

void add_RenamedDeprecatedStruct_binding(nb::handle mod) {
    nb::class_<ns::RenamedDeprecatedStruct>(mod, "RenamedDeprecatedStruct")
        .def(nb::init<>());
}


}
