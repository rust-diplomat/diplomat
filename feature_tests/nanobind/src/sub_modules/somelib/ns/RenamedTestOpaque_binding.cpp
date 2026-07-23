#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedTestOpaque.hpp"

namespace somelib::ns {
void add_RenamedTestOpaque_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedTestOpaque> opaque(mod, "RenamedTestOpaque");
}

} 