#include "diplomat_nanobind_common.hpp"


#include "MutableCallbackHolder.hpp"

namespace somelib {
void add_MutableCallbackHolder_binding(nb::module_ mod) {
    PyType_Slot somelib_MutableCallbackHolder_slots[] = {
        {Py_tp_free, (void *)somelib::MutableCallbackHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::MutableCallbackHolder> opaque(mod, "MutableCallbackHolder", nb::type_slots(somelib_MutableCallbackHolder_slots));
    opaque
        .def("call", &somelib::MutableCallbackHolder::call, "a"_a)
        .def(nb::new_(&somelib::MutableCallbackHolder::new_), "func"_a);
}

} 