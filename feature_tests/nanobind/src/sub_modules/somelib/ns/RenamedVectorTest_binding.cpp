#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedVectorTest.hpp"

namespace somelib::ns {
void add_RenamedVectorTest_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedVectorTest> opaque(mod, "RenamedVectorTest");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedVectorTest::new_))))
        .def("__getitem__", [](somelib::ns::RenamedVectorTest* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "idx"_a)
        .def_prop_ro("len", &somelib::ns::RenamedVectorTest::len)
        .def("push", &somelib::ns::RenamedVectorTest::push, "value"_a);
}

} 