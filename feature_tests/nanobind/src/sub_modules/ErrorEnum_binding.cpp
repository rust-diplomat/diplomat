#include "diplomat_nanobind_common.hpp"


#include "ErrorEnum.hpp"


void add_ErrorEnum_binding(nb::module_ mod) {
    nb::class_<ErrorEnum> e_class(mod, "ErrorEnum");
    
        nb::enum_<ErrorEnum::Value>(e_class, "ErrorEnum")
            .value("Foo", ErrorEnum::Foo)
            .value("Bar", ErrorEnum::Bar)
            .export_values();
    
        e_class
            .def(nb::init_implicit<ErrorEnum::Value>())
            .def(nb::self == ErrorEnum::Value())
            .def("__repr__", [](const ErrorEnum& self){
                return nb::str(nb::cast(ErrorEnum::Value(self)));
            });
    
}

