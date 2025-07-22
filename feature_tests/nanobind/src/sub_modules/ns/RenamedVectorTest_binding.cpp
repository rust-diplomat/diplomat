#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedVectorTest.hpp"


namespace ns{

void add_RenamedVectorTest_binding(nb::handle mod) {
    PyType_Slot ns_RenamedVectorTest_slots[] = {
        {Py_tp_free, (void *)ns::RenamedVectorTest::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedVectorTest>(mod, "RenamedVectorTest", nb::type_slots(ns_RenamedVectorTest_slots))
    	.def("__getitem__", &ns::RenamedVectorTest::operator[], "idx"_a)
    	.def_prop_ro("len", &ns::RenamedVectorTest::len)
    	.def(nb::new_(&ns::RenamedVectorTest::new_))
    	.def("push", &ns::RenamedVectorTest::push, "val"_a);
}


}
