#include "diplomat_nanobind_common.hpp"


#include "OpaqueMut.hpp"

namespace somelib {
void add_OpaqueMut_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueMut_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueMut::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueMut> opaque(mod, "OpaqueMut", nb::type_slots(somelib_OpaqueMut_slots));
}

} 