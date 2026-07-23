#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMixinTest.hpp"

namespace somelib::ns {
void add_RenamedMixinTest_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedMixinTest> opaque(mod, "RenamedMixinTest");
    opaque
        .def_static("hello", &somelib::ns::RenamedMixinTest::hello);
}

} 