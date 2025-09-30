#include "diplomat_nanobind_common.hpp"


#include "OptionString.hpp"

namespace somelib {
void add_OptionString_binding(nb::module_ mod) {
    PyType_Slot somelib_OptionString_slots[] = {
        {Py_tp_free, (void *)somelib::OptionString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OptionString>(mod, "OptionString", nb::type_slots(somelib_OptionString_slots))
        .def("borrow", &somelib::OptionString::borrow)
        .def_static("new", &somelib::OptionString::new_, "diplomat_str"_a)
        .def("write", &somelib::OptionString::write);
}

} 