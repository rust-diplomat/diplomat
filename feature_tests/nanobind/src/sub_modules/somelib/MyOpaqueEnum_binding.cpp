#include "diplomat_nanobind_common.hpp"


#include "MyOpaqueEnum.hpp"

namespace somelib {
void add_MyOpaqueEnum_binding(nb::module_ mod) {
    nb::class_<somelib::MyOpaqueEnum> opaque(mod, "MyOpaqueEnum");
    opaque
        .def_static("new", std::move(maybe_op_unwrap(&somelib::MyOpaqueEnum::new_)))
        .def("__str__", &somelib::MyOpaqueEnum::to_string);
}

} 