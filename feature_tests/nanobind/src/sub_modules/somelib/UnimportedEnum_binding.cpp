#include "diplomat_nanobind_common.hpp"


#include "UnimportedEnum.hpp"

namespace somelib {
void add_UnimportedEnum_binding(nb::module_ mod) {
    nb::class_<somelib::UnimportedEnum> e_class(mod, "UnimportedEnum");
    
        nb::enum_<somelib::UnimportedEnum::Value> enumerator(e_class, "UnimportedEnum");
        enumerator
            .value("A", somelib::UnimportedEnum::A)
            .value("B", somelib::UnimportedEnum::B)
            .value("C", somelib::UnimportedEnum::C)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::UnimportedEnum::Value>())
            .def(nb::self == somelib::UnimportedEnum::Value())
            .def("__repr__", [](const somelib::UnimportedEnum& self){
                return nb::str(nb::cast(somelib::UnimportedEnum::Value(self)));
            });
}

} 