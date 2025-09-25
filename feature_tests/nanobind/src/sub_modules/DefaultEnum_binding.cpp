#include "diplomat_nanobind_common.hpp"


#include "DefaultEnum.hpp"


void add_DefaultEnum_binding(nb::module_ mod) {
    nb::class_<DefaultEnum> e_class(mod, "DefaultEnum");
    
        nb::enum_<DefaultEnum::Value>(e_class, "DefaultEnum")
            .value("A", DefaultEnum::A)
            .value("B", DefaultEnum::B)
            .export_values();
    
        e_class
            .def(nb::init_implicit<DefaultEnum::Value>())
            .def(nb::self == DefaultEnum::Value())
            .def("__repr__", [](const DefaultEnum& self){
                return nb::str(nb::cast(DefaultEnum::Value(self)));
            });
    
}

