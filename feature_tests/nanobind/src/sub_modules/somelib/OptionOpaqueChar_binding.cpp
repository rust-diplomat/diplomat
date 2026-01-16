#include "diplomat_nanobind_common.hpp"


#include "OptionOpaqueChar.hpp"

namespace somelib {
void add_OptionOpaqueChar_binding(nb::module_ mod) {
    PyType_Slot somelib_OptionOpaqueChar_slots[] = {
        {Py_tp_free, (void *)somelib::OptionOpaqueChar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OptionOpaqueChar> opaque(mod, "OptionOpaqueChar", nb::type_slots(somelib_OptionOpaqueChar_slots));
    opaque
        .def("assert_char", &somelib::OptionOpaqueChar::assert_char, "ch"_a);
}

} 