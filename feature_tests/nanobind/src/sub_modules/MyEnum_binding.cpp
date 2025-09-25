#include "diplomat_nanobind_common.hpp"


#include "MyEnum.hpp"


void add_MyEnum_binding(nb::module_ mod) {
    nb::class_<MyEnum> e_class(mod, "MyEnum");
    
        nb::enum_<MyEnum::Value>(e_class, "MyEnum")
            .value("A", MyEnum::A)
            .value("B", MyEnum::B)
            .value("C", MyEnum::C)
            .value("D", MyEnum::D)
            .value("E", MyEnum::E)
            .value("F", MyEnum::F)
            .export_values();
    
        e_class
            .def(nb::init_implicit<MyEnum::Value>())
            .def(nb::self == MyEnum::Value())
            .def("__repr__", [](const MyEnum& self){
                return nb::str(nb::cast(MyEnum::Value(self)));
            });
    
}

