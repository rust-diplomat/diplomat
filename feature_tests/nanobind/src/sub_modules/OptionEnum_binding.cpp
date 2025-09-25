#include "diplomat_nanobind_common.hpp"


#include "OptionEnum.hpp"


void add_OptionEnum_binding(nb::module_ mod) {
    nb::class_<OptionEnum> e_class(mod, "OptionEnum");
    
        nb::enum_<OptionEnum::Value>(e_class, "OptionEnum")
            .value("Foo", OptionEnum::Foo)
            .value("Bar", OptionEnum::Bar)
            .value("Baz", OptionEnum::Baz)
            .export_values();
    
        e_class
            .def(nb::init_implicit<OptionEnum::Value>())
            .def(nb::self == OptionEnum::Value())
            .def("__repr__", [](const OptionEnum& self){
                return nb::str(nb::cast(OptionEnum::Value(self)));
            });
    
}

