#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecated.hpp"


namespace ns{

void add_RenamedDeprecated_binding(nb::handle mod) {
    nb::class_<ns::RenamedDeprecated>(mod, "RenamedDeprecated")
        .def(nb::init<>());
}


}
