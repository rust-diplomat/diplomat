#include "diplomat_nanobind_common.hpp"


#include "PrimitiveStruct.hpp"
#include "PrimitiveStructVec.hpp"
#include "ns/RenamedStructWithAttrs.hpp"

namespace somelib {
void add_PrimitiveStructVec_binding(nb::module_ mod) {
    PyType_Slot somelib_PrimitiveStructVec_slots[] = {
        {Py_tp_free, (void *)somelib::PrimitiveStructVec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::PrimitiveStructVec> opaque(mod, "PrimitiveStructVec", nb::type_slots(somelib_PrimitiveStructVec_slots));
    opaque
        .def("__getitem__", &somelib::PrimitiveStructVec::__getitem__, "idx"_a)
        .def("__len__", &somelib::PrimitiveStructVec::__len__)
        .def("append", &somelib::PrimitiveStructVec::append, "value"_a)
        .def_prop_ro("asSlice", &somelib::PrimitiveStructVec::as_slice)
        .def_prop_ro("asSliceMut", &somelib::PrimitiveStructVec::as_slice_mut)
        .def(nb::new_(&somelib::PrimitiveStructVec::new_))
        .def_static("take_slice_from_other_namespace", &somelib::PrimitiveStructVec::take_slice_from_other_namespace, "_sl"_a);
}

} 