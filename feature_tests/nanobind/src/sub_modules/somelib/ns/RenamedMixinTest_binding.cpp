#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMixinTest.hpp"

namespace somelib::ns {
void add_RenamedMixinTest_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedMixinTest_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedMixinTest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedMixinTest> opaque(mod, "RenamedMixinTest", nb::type_slots(somelib_ns_RenamedMixinTest_slots));
    opaque
        .def_static("hello", &somelib::ns::RenamedMixinTest::hello);
}

} 