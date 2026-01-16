#include "diplomat_nanobind_common.hpp"


#include "CallbackHolder.hpp"

namespace somelib {
void add_CallbackHolder_binding(nb::module_ mod) {
    PyType_Slot somelib_CallbackHolder_slots[] = {
        {Py_tp_free, (void *)somelib::CallbackHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::CallbackHolder> opaque(mod, "CallbackHolder", nb::type_slots(somelib_CallbackHolder_slots));
    opaque
        .def("call", &somelib::CallbackHolder::call, "a"_a)
        .def(nb::new_(&somelib::CallbackHolder::new_), "func"_a);
}

} 