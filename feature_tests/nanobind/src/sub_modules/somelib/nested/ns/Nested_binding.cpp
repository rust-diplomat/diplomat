#include "diplomat_nanobind_common.hpp"


#include "nested/ns/Nested.hpp"

namespace somelib::nested::ns {
void add_Nested_binding(nb::module_ mod) {
    nb::class_<somelib::nested::ns::Nested> opaque(mod, "Nested");
}

} 