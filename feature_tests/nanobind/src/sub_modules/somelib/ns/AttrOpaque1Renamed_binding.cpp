#include "diplomat_nanobind_common.hpp"


#include "Unnamespaced.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"

namespace somelib::ns {
void add_AttrOpaque1Renamed_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_AttrOpaque1Renamed_slots[] = {
        {Py_tp_free, (void *)somelib::ns::AttrOpaque1Renamed::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::AttrOpaque1Renamed> opaque(mod, "AttrOpaque1Renamed", nb::type_slots(somelib_ns_AttrOpaque1Renamed_slots));
    opaque
        .def_prop_ro("abirenamed", &somelib::ns::AttrOpaque1Renamed::abirenamed)
        .def_static("hello", &somelib::ns::AttrOpaque1Renamed::hello)
        .def_static("mac_test", &somelib::ns::AttrOpaque1Renamed::mac_test)
        .def_prop_ro("method", &somelib::ns::AttrOpaque1Renamed::method_renamed)
        .def_static("test_namespaced_callback", &somelib::ns::AttrOpaque1Renamed::test_namespaced_callback, "_t"_a)
        .def(nb::new_(&somelib::ns::AttrOpaque1Renamed::totally_not_new))
        .def("use_namespaced", &somelib::ns::AttrOpaque1Renamed::use_namespaced, "_n"_a)
        .def("use_unnamespaced", &somelib::ns::AttrOpaque1Renamed::use_unnamespaced, "_un"_a);
}

} 