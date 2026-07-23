#include "diplomat_nanobind_common.hpp"


#include "nested/ns2/Nested.hpp"

namespace somelib::nested::ns2 {
void add_Nested_binding(nb::module_ mod) {
    nb::class_<somelib::nested::ns2::Nested> opaque(mod, "Nested");
}

} 