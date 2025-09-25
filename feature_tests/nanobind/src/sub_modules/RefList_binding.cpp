#include "diplomat_nanobind_common.hpp"


#include "RefList.hpp"
#include "RefListParameter.hpp"


void add_RefList_binding(nb::module_ mod) {
    PyType_Slot RefList_slots[] = {
        {Py_tp_free, (void *)RefList::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<RefList>(mod, "RefList", nb::type_slots(RefList_slots))
        .def_static("node", &RefList::node, "data"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    ;
}

