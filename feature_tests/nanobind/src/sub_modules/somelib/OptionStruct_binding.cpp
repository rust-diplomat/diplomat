#include "diplomat_nanobind_common.hpp"


#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionStruct.hpp"

namespace somelib {
void add_OptionStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OptionStruct> st(mod, "OptionStruct");
    st
        .def(nb::init<>())
        .def(nb::init<std::unique_ptr<somelib::OptionOpaque>, std::unique_ptr<somelib::OptionOpaqueChar>, uint32_t, std::unique_ptr<somelib::OptionOpaque>>(), "a"_a,  "b"_a,  "c"_a.none(),  "d"_a)
        .def_prop_rw("a", 
            [](const somelib::OptionStruct& self) { return self.a.get(); },
            [](somelib::OptionStruct& self, std::unique_ptr<somelib::OptionOpaque>&& v) { self.a = std::move(v); }
        )
        .def_prop_rw("b", 
            [](const somelib::OptionStruct& self) { return self.b.get(); },
            [](somelib::OptionStruct& self, std::unique_ptr<somelib::OptionOpaqueChar>&& v) { self.b = std::move(v); }
        )
        .def_rw("c", &somelib::OptionStruct::c)
        .def_prop_rw("d", 
            [](const somelib::OptionStruct& self) { return self.d.get(); },
            [](somelib::OptionStruct& self, std::unique_ptr<somelib::OptionOpaque>&& v) { self.d = std::move(v); }
        );
}

} 