#include "diplomat_nanobind_common.hpp"


#include "RefList.hpp"
#include "RefListParameter.hpp"

namespace somelib {
void add_RefList_binding(nb::module_ mod) {
    PyType_Slot somelib_RefList_slots[] = {
        {Py_tp_free, (void *)somelib::RefList::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::RefList> opaque(mod, "RefList", nb::type_slots(somelib_RefList_slots));
    opaque
        .def_static("node", &somelib::RefList::node, "data"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    ;
}

} 