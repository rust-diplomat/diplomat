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
        .def(nb::new_(swap_lvalue_wrap(std::move(maybe_op_unwrap(&somelib::OpaqueCallbacks::ctor)))), "f"_a, "st"_a)
        .def("opaque_cb_mut_self", swap_lvalue_wrap(&somelib::OpaqueCallbacks::opaque_cb_mut_self), "cb"_a, "st"_a)
        .def("opaque_cb_self", swap_lvalue_wrap(&somelib::OpaqueCallbacks::opaque_cb_self), "cb"_a, "st"_a)
        .def_static("ret_op", swap_lvalue_wrap(&somelib::OpaqueCallbacks::ret_op), "f"_a, "st"_a, nb::rv_policy::reference);
}

} 