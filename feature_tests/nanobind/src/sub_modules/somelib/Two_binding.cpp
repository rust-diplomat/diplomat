#include "diplomat_nanobind_common.hpp"


#include "Two.hpp"

namespace somelib {
void add_Two_binding(nb::module_ mod) {
    nb::class_<somelib::Two> opaque(mod, "Two");
}

} 