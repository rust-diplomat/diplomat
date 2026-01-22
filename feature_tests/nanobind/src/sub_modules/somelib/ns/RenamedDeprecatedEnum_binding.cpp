#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedEnum.hpp"

namespace somelib::ns {
void add_RenamedDeprecatedEnum_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedDeprecatedEnum> e_class(mod, "RenamedDeprecatedEnum");
    
        nb::enum_<somelib::ns::RenamedDeprecatedEnum::Value> enumerator(e_class, "RenamedDeprecatedEnum");
        enumerator
            .value("A", somelib::ns::RenamedDeprecatedEnum::A)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::ns::RenamedDeprecatedEnum::Value>())
            .def(nb::self == somelib::ns::RenamedDeprecatedEnum::Value())
            .def("__repr__", [](const somelib::ns::RenamedDeprecatedEnum& self){
                return nb::str(nb::cast(somelib::ns::RenamedDeprecatedEnum::Value(self)));
            });
}

} 