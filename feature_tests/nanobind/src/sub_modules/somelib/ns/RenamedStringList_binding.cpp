#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedStringList.hpp"

namespace somelib::ns {
void add_RenamedStringList_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedStringList_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedStringList::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedStringList> opaque(mod, "RenamedStringList", nb::type_slots(somelib_ns_RenamedStringList_slots));
    opaque
        .def_static("return_new", &somelib::ns::RenamedStringList::return_new);
}

} 