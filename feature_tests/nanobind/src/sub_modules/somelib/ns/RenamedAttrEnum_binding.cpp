#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedAttrEnum.hpp"

namespace somelib::ns {
void add_RenamedAttrEnum_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedAttrEnum> e_class(mod, "RenamedAttrEnum");
    
        nb::enum_<somelib::ns::RenamedAttrEnum::Value> enumerator(e_class, "RenamedAttrEnum");
        enumerator
            .value("A", somelib::ns::RenamedAttrEnum::A)
            .value("B", somelib::ns::RenamedAttrEnum::B)
            .value("Renamed", somelib::ns::RenamedAttrEnum::Renamed)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::ns::RenamedAttrEnum::Value>())
            .def(nb::self == somelib::ns::RenamedAttrEnum::Value())
            .def("__repr__", [](const somelib::ns::RenamedAttrEnum& self){
                return nb::str(nb::cast(somelib::ns::RenamedAttrEnum::Value(self)));
            });
}

} 