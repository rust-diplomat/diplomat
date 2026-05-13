#include "diplomat_nanobind_common.hpp"


#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionStruct.hpp"

namespace somelib {
void add_OptionStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OptionStruct> st(mod, "OptionStruct");
    st
        .def_prop_ro("a",
            [](const somelib::OptionStruct& self) { return self.a.get(); }
        )
        .def_prop_ro("b",
            [](const somelib::OptionStruct& self) { return self.b.get(); }
        )
        .def_ro("c", &somelib::OptionStruct::c)
        .def_prop_ro("d",
            [](const somelib::OptionStruct& self) { return self.d.get(); }
        );
}

} 