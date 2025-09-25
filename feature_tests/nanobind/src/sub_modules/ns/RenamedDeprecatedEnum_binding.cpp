#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedEnum.hpp"


namespace ns{

void add_RenamedDeprecatedEnum_binding(nb::module_ mod) {
    nb::class_<ns::RenamedDeprecatedEnum> e_class(mod, "RenamedDeprecatedEnum");
    
        nb::enum_<ns::RenamedDeprecatedEnum::Value>(e_class, "RenamedDeprecatedEnum")
            .value("A", ns::RenamedDeprecatedEnum::A)
            .export_values();
    
        e_class
            .def(nb::init_implicit<ns::RenamedDeprecatedEnum::Value>())
            .def(nb::self == ns::RenamedDeprecatedEnum::Value())
            .def("__repr__", [](const ns::RenamedDeprecatedEnum& self){
                return nb::str(nb::cast(ns::RenamedDeprecatedEnum::Value(self)));
            });
    
}


}
