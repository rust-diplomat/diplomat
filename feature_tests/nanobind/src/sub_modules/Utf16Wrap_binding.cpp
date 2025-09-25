#include "diplomat_nanobind_common.hpp"


#include "Utf16Wrap.hpp"


void add_Utf16Wrap_binding(nb::module_ mod) {
    PyType_Slot Utf16Wrap_slots[] = {
        {Py_tp_free, (void *)Utf16Wrap::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Utf16Wrap>(mod, "Utf16Wrap", nb::type_slots(Utf16Wrap_slots))
        .def("borrow_cont", &Utf16Wrap::borrow_cont)
        .def(nb::new_(&Utf16Wrap::from_utf16), "input"_a)
        .def("get_debug_str", &Utf16Wrap::get_debug_str);
}

