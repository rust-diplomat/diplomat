#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedVectorTest.hpp"

namespace somelib::ns {
void add_RenamedVectorTest_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedVectorTest_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedVectorTest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedVectorTest> opaque(mod, "RenamedVectorTest", nb::type_slots(somelib_ns_RenamedVectorTest_slots));
    opaque
        .def("__getitem__", [](somelib::ns::RenamedVectorTest* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "idx"_a)
        .def_prop_ro("len", &somelib::ns::RenamedVectorTest::len)
        .def(nb::new_(&somelib::ns::RenamedVectorTest::new_))
        .def("push", &somelib::ns::RenamedVectorTest::push, "value"_a);
}

} 