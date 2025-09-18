#include "diplomat_nanobind_common.hpp"


#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionStruct.hpp"


void add_OptionStruct_binding(nb::module_ mod) {
    nb::class_<OptionStruct>(mod, "OptionStruct")
        .def(nb::init<>())
        .def(nb::init<std::unique_ptr<OptionOpaque>, std::unique_ptr<OptionOpaqueChar>, uint32_t, std::unique_ptr<OptionOpaque>>(), "a"_a,  "b"_a,  "c"_a.none(),  "d"_a)
        .def_prop_rw("a", 
            [](const OptionStruct& self) { return self.a.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaque>&& v) { self.a = std::move(v); }
        )
        .def_prop_rw("b", 
            [](const OptionStruct& self) { return self.b.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaqueChar>&& v) { self.b = std::move(v); }
        )
        .def_rw("c", &OptionStruct::c)
        .def_prop_rw("d", 
            [](const OptionStruct& self) { return self.d.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaque>&& v) { self.d = std::move(v); }
        );
}

