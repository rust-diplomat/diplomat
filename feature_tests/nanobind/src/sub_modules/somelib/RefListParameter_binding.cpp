#include "diplomat_nanobind_common.hpp"


#include "RefListParameter.hpp"

namespace somelib {
void add_RefListParameter_binding(nb::module_ mod) {
    nb::class_<somelib::RefListParameter> opaque(mod, "RefListParameter");
}

} 