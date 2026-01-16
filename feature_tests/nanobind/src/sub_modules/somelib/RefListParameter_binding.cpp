#include "diplomat_nanobind_common.hpp"


#include "RefListParameter.hpp"

namespace somelib {
void add_RefListParameter_binding(nb::module_ mod) {
    PyType_Slot somelib_RefListParameter_slots[] = {
        {Py_tp_free, (void *)somelib::RefListParameter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::RefListParameter> opaque(mod, "RefListParameter", nb::type_slots(somelib_RefListParameter_slots));
    ;
}

} 