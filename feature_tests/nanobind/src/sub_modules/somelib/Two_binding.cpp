#include "diplomat_nanobind_common.hpp"


#include "Two.hpp"

namespace somelib {
void add_Two_binding(nb::module_ mod) {
    PyType_Slot somelib_Two_slots[] = {
        {Py_tp_free, (void *)somelib::Two::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Two> opaque(mod, "Two", nb::type_slots(somelib_Two_slots));
    ;
}

} 