#include "diplomat_nanobind_common.hpp"


#include "Utf16Wrap.hpp"

namespace somelib {
void add_Utf16Wrap_binding(nb::module_ mod) {
    PyType_Slot somelib_Utf16Wrap_slots[] = {
        {Py_tp_free, (void *)somelib::Utf16Wrap::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Utf16Wrap> opaque(mod, "Utf16Wrap", nb::type_slots(somelib_Utf16Wrap_slots));
    opaque
        .def("borrow_cont", &somelib::Utf16Wrap::borrow_cont)
        .def(nb::new_(&somelib::Utf16Wrap::from_utf16), "input"_a)
        .def("get_debug_str", &somelib::Utf16Wrap::get_debug_str);
}

} 