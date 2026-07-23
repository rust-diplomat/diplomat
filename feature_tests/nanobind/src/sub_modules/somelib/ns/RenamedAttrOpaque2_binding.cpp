#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedAttrOpaque2.hpp"

namespace somelib::ns {
void add_RenamedAttrOpaque2_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedAttrOpaque2> opaque(mod, "RenamedAttrOpaque2");
}

} 