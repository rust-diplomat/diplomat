#include "diplomat_nanobind_common.hpp"


#include "Unnamespaced.hpp"


void add_Unnamespaced_binding(nb::handle mod) {
    PyType_Slot Unnamespaced_slots[] = {
        {Py_tp_free, (void *)Unnamespaced::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Unnamespaced>(mod, "Unnamespaced", nb::type_slots(Unnamespaced_slots))
    	.def_static("make", &Unnamespaced::make, "_e"_a ) // unsupported special method NamedConstructor(None)
    	.def("use_namespaced", &Unnamespaced::use_namespaced, "_n"_a);
}

