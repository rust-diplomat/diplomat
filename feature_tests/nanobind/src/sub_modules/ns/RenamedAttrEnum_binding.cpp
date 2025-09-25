#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedAttrEnum.hpp"


namespace ns{

void add_RenamedAttrEnum_binding(nb::module_ mod) {
    nb::class_<ns::RenamedAttrEnum> e_class(mod, "RenamedAttrEnum");
    
        nb::enum_<ns::RenamedAttrEnum::Value>(e_class, "RenamedAttrEnum")
            .value("A", ns::RenamedAttrEnum::A)
            .value("B", ns::RenamedAttrEnum::B)
            .value("Renamed", ns::RenamedAttrEnum::Renamed)
            .export_values();
    
        e_class
            .def(nb::init_implicit<ns::RenamedAttrEnum::Value>())
            .def(nb::self == ns::RenamedAttrEnum::Value())
            .def("__repr__", [](const ns::RenamedAttrEnum& self){
                return nb::str(nb::cast(ns::RenamedAttrEnum::Value(self)));
            });
    
}


}
