#include "diplomat_nanobind_common.hpp"


#include "DefaultEnum.hpp"

namespace somelib {
void add_DefaultEnum_binding(nb::module_ mod) {
    nb::class_<somelib::DefaultEnum> e_class(mod, "DefaultEnum");
    
        nb::enum_<somelib::DefaultEnum::Value> enumerator(e_class, "DefaultEnum");
        enumerator
            .value("A", somelib::DefaultEnum::A)
            .value("B", somelib::DefaultEnum::B)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::DefaultEnum::Value>())
            .def(nb::self == somelib::DefaultEnum::Value())
            .def("__repr__", [](const somelib::DefaultEnum& self){
                return nb::str(nb::cast(somelib::DefaultEnum::Value(self)));
            });
}

} 