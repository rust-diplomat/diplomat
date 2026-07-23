#include "diplomat_nanobind_common.hpp"


#include "Bar.hpp"

namespace somelib {
void add_Bar_binding(nb::module_ mod) {
    nb::class_<somelib::Bar> opaque(mod, "Bar");
    opaque
        .def_prop_ro("foo", &somelib::Bar::foo);
}

} 