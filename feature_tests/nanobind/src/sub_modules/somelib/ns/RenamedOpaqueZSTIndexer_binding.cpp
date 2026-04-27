#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueZSTIndexer.hpp"

namespace somelib::ns {
void add_RenamedOpaqueZSTIndexer_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedOpaqueZSTIndexer_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedOpaqueZSTIndexer::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedOpaqueZSTIndexer> opaque(mod, "RenamedOpaqueZSTIndexer", nb::type_slots(somelib_ns_RenamedOpaqueZSTIndexer_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIndexer::new_))))
        .def("__getitem__", [](somelib::ns::RenamedOpaqueZSTIndexer* self, size_t index) {
                auto out = map_inner(self->operator[] (index));
                if (out.get() == nullptr) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }}, "idx"_a);
}

} 