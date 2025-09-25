#include "diplomat_nanobind_common.hpp"


#include "UnimportedEnum.hpp"


void add_UnimportedEnum_binding(nb::module_ mod) {
    nb::class_<UnimportedEnum> e_class(mod, "UnimportedEnum");
    
        nb::enum_<UnimportedEnum::Value>(e_class, "UnimportedEnum")
            .value("A", UnimportedEnum::A)
            .value("B", UnimportedEnum::B)
            .value("C", UnimportedEnum::C)
            .export_values();
    
        e_class
            .def(nb::init_implicit<UnimportedEnum::Value>())
            .def(nb::self == UnimportedEnum::Value())
            .def("__repr__", [](const UnimportedEnum& self){
                return nb::str(nb::cast(UnimportedEnum::Value(self)));
            });
    
}

