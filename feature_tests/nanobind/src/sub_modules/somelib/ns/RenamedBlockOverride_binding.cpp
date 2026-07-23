#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedBlockOverride.hpp"

namespace somelib::ns {
void add_RenamedBlockOverride_binding(nb::module_ mod) {
    
    //Pre-Init Test
    nb::class_<somelib::ns::RenamedBlockOverride> opaque(mod, "RenamedBlockOverride");
    
    opaque.def("special_function", &somelib::ns::RenamedBlockOverride::special_function);
}

} 