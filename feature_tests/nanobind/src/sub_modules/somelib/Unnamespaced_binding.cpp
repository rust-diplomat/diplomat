#include "diplomat_nanobind_common.hpp"


#include "Unnamespaced.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"

namespace somelib {
void add_Unnamespaced_binding(nb::module_ mod) {
    nb::class_<somelib::Unnamespaced> opaque(mod, "Unnamespaced");
    opaque
        .def_static("make", std::move(maybe_op_unwrap(&somelib::Unnamespaced::make)), "_e"_a)
        .def("use_namespaced", &somelib::Unnamespaced::use_namespaced, "_n"_a);
}

} 