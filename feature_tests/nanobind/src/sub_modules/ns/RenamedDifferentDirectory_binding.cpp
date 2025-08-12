#include "diplomat_nanobind_common.hpp"


#include "test_directory/RenamedDifferentDirectory.hpp"


namespace ns{

void add_RenamedDifferentDirectory_binding(nb::handle mod) {
    PyType_Slot ns_RenamedDifferentDirectory_slots[] = {
        {Py_tp_free, (void *)ns::RenamedDifferentDirectory::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedDifferentDirectory>(mod, "RenamedDifferentDirectory", nb::type_slots(ns_RenamedDifferentDirectory_slots));
}


}
