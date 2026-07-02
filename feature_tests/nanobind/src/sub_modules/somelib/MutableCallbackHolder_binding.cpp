#include "diplomat_nanobind_common.hpp"


#include "MutableCallbackHolder.hpp"
#include "MyString.hpp"

namespace somelib {
void add_MutableCallbackHolder_binding(nb::module_ mod) {
    PyType_Slot somelib_MutableCallbackHolder_slots[] = {
        {Py_tp_free, (void *)somelib::MutableCallbackHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::MutableCallbackHolder> opaque(mod, "MutableCallbackHolder", nb::type_slots(somelib_MutableCallbackHolder_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::MutableCallbackHolder::new_))), "func"_a)
        .def("call", &somelib::MutableCallbackHolder::call, "a"_a)
        .def("opaque_cb_mut_self", swap_lvalue_wrap(&somelib::MutableCallbackHolder::opaque_cb_mut_self), "cb"_a, "st"_a)
        .def("opaque_cb_self", swap_lvalue_wrap(&somelib::MutableCallbackHolder::opaque_cb_self), "cb"_a, "st"_a);
}

} 