#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedStringList.hpp"

namespace somelib::ns {
void add_RenamedStringList_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedStringList> opaque(mod, "RenamedStringList", "Testing support for List[str] in Nanobind");
    
    opaque.def_static("return_new", &somelib::ns::RenamedStringList::return_new);
    
}

} 