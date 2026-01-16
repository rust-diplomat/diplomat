#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIndexer.hpp"

namespace somelib::ns {
void add_RenamedMyIndexer_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedMyIndexer_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedMyIndexer::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedMyIndexer> opaque(mod, "RenamedMyIndexer", nb::type_slots(somelib_ns_RenamedMyIndexer_slots));
    opaque
        .def("__getitem__", [](somelib::ns::RenamedMyIndexer* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "i"_a);
}

} 