#include "diplomat_nanobind_common.hpp"


#include "Bar.hpp"

namespace somelib {
void add_Bar_binding(nb::module_ mod) {
    PyType_Slot somelib_Bar_slots[] = {
        {Py_tp_free, (void *)somelib::Bar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Bar> opaque(mod, "Bar", nb::type_slots(somelib_Bar_slots));
    opaque
        .def_prop_ro("foo", &somelib::Bar::foo);
}

} 