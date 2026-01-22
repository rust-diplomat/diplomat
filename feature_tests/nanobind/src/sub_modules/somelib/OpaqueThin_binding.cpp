#include "diplomat_nanobind_common.hpp"


#include "OpaqueThin.hpp"

namespace somelib {
void add_OpaqueThin_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueThin_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueThin::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueThin> opaque(mod, "OpaqueThin", nb::type_slots(somelib_OpaqueThin_slots));
    opaque
        .def_prop_ro("a", &somelib::OpaqueThin::a)
        .def_prop_ro("b", &somelib::OpaqueThin::b)
        .def_prop_ro("c", &somelib::OpaqueThin::c);
}

} 