#include "diplomat_nanobind_common.hpp"


#include "ErrorEnum.hpp"

namespace somelib {
void add_ErrorEnum_binding(nb::module_ mod) {
    nb::class_<somelib::ErrorEnum> e_class(mod, "ErrorEnum");
    
        nb::enum_<somelib::ErrorEnum::Value> enumerator(e_class, "ErrorEnum");
        enumerator
            .value("Foo", somelib::ErrorEnum::Foo)
            .value("Bar", somelib::ErrorEnum::Bar)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::ErrorEnum::Value>())
            .def(nb::self == somelib::ErrorEnum::Value())
            .def("__repr__", [](const somelib::ErrorEnum& self){
                return nb::str(nb::cast(somelib::ErrorEnum::Value(self)));
            });
}

} 