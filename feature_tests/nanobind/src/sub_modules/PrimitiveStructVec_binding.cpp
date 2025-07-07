#include "diplomat_nanobind_common.hpp"


#include "PrimitiveStructVec.hpp"


void add_PrimitiveStructVec_binding(nb::handle mod) {
    PyType_Slot PrimitiveStructVec_slots[] = {
        {Py_tp_free, (void *)PrimitiveStructVec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<PrimitiveStructVec>(mod, "PrimitiveStructVec", nb::type_slots(PrimitiveStructVec_slots))
    	.def("__getitem__", &PrimitiveStructVec::__getitem__, "idx"_a)
    	.def("__len__", &PrimitiveStructVec::__len__)
    	.def("append", &PrimitiveStructVec::append, "value"_a)
    	.def_prop_ro("asSlice", &PrimitiveStructVec::as_slice)
    	.def_prop_ro("asSliceMut", &PrimitiveStructVec::as_slice_mut)
    	.def(nb::new_(&PrimitiveStructVec::new_));
}

