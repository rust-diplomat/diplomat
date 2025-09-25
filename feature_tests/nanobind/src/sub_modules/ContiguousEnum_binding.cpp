#include "diplomat_nanobind_common.hpp"


#include "ContiguousEnum.hpp"


void add_ContiguousEnum_binding(nb::module_ mod) {
    nb::class_<ContiguousEnum> e_class(mod, "ContiguousEnum");
    
        nb::enum_<ContiguousEnum::Value>(e_class, "ContiguousEnum")
            .value("C", ContiguousEnum::C)
            .value("D", ContiguousEnum::D)
            .value("E", ContiguousEnum::E)
            .value("F", ContiguousEnum::F)
            .export_values();
    
        e_class
            .def(nb::init_implicit<ContiguousEnum::Value>())
            .def(nb::self == ContiguousEnum::Value())
            .def("__repr__", [](const ContiguousEnum& self){
                return nb::str(nb::cast(ContiguousEnum::Value(self)));
            });
    
}

