#include "diplomat_nanobind_common.hpp"


#include "Unnamespaced.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"

namespace somelib {
void add_Unnamespaced_binding(nb::module_ mod) {
    PyType_Slot somelib_Unnamespaced_slots[] = {
        {Py_tp_free, (void *)somelib::Unnamespaced::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Unnamespaced> opaque(mod, "Unnamespaced", nb::type_slots(somelib_Unnamespaced_slots));
    opaque
        .def_static("make", &somelib::Unnamespaced::make, "_e"_a ) // unsupported special method NamedConstructor(None)
        .def("use_namespaced", &somelib::Unnamespaced::use_namespaced, "_n"_a);
}

} 