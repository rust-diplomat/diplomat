#include "diplomat_nanobind_common.hpp"


#include "MyEnum.hpp"

namespace somelib {
void add_MyEnum_binding(nb::module_ mod) {
    nb::class_<somelib::MyEnum> e_class(mod, "MyEnum");
    
        nb::enum_<somelib::MyEnum::Value> enumerator(e_class, "MyEnum");
        enumerator
            .value("A", somelib::MyEnum::A)
            .value("B", somelib::MyEnum::B)
            .value("C", somelib::MyEnum::C)
            .value("D", somelib::MyEnum::D)
            .value("E", somelib::MyEnum::E)
            .value("F", somelib::MyEnum::F)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::MyEnum::Value>())
            .def(nb::self == somelib::MyEnum::Value())
            .def("__repr__", [](const somelib::MyEnum& self){
                return nb::str(nb::cast(somelib::MyEnum::Value(self)));
            });
}

} 