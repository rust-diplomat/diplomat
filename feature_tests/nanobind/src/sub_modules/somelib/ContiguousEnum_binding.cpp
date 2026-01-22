#include "diplomat_nanobind_common.hpp"


#include "ContiguousEnum.hpp"

namespace somelib {
void add_ContiguousEnum_binding(nb::module_ mod) {
    nb::class_<somelib::ContiguousEnum> e_class(mod, "ContiguousEnum");
    
        nb::enum_<somelib::ContiguousEnum::Value> enumerator(e_class, "ContiguousEnum");
        enumerator
            .value("C", somelib::ContiguousEnum::C)
            .value("D", somelib::ContiguousEnum::D)
            .value("E", somelib::ContiguousEnum::E)
            .value("F", somelib::ContiguousEnum::F)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::ContiguousEnum::Value>())
            .def(nb::self == somelib::ContiguousEnum::Value())
            .def("__repr__", [](const somelib::ContiguousEnum& self){
                return nb::str(nb::cast(somelib::ContiguousEnum::Value(self)));
            });
}

} 