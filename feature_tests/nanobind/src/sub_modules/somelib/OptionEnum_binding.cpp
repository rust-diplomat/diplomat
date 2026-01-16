#include "diplomat_nanobind_common.hpp"


#include "OptionEnum.hpp"

namespace somelib {
void add_OptionEnum_binding(nb::module_ mod) {
    nb::class_<somelib::OptionEnum> e_class(mod, "OptionEnum");
    
        nb::enum_<somelib::OptionEnum::Value> enumerator(e_class, "OptionEnum");
        enumerator
            .value("Foo", somelib::OptionEnum::Foo)
            .value("Bar", somelib::OptionEnum::Bar)
            .value("Baz", somelib::OptionEnum::Baz)
            .export_values();
    
        e_class
            .def(nb::init_implicit<somelib::OptionEnum::Value>())
            .def(nb::self == somelib::OptionEnum::Value())
            .def("__repr__", [](const somelib::OptionEnum& self){
                return nb::str(nb::cast(somelib::OptionEnum::Value(self)));
            });
}

} 