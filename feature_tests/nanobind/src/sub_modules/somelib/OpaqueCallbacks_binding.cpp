#include "diplomat_nanobind_common.hpp"


#include "MyString.hpp"
#include "OpaqueCallbacks.hpp"

namespace somelib {
void add_OpaqueCallbacks_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueCallbacks_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueCallbacks::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueCallbacks> opaque(mod, "OpaqueCallbacks", nb::type_slots(somelib_OpaqueCallbacks_slots));
    opaque
        .def_static("ret_op", &somelib::OpaqueCallbacks::ret_op, "f"_a, "st"_a, nb::rv_policy::reference);
}

} 