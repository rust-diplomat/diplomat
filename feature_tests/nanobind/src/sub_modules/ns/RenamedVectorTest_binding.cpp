#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedVectorTest.hpp"


namespace ns{

void add_RenamedVectorTest_binding(nb::handle mod) {
    nb::class_<ns::RenamedVectorTest>(mod, "RenamedVectorTest")
        .def(nb::init<>())
        .def(nb::init<double>(), "test"_a.none())
        .def_rw("test", &ns::RenamedVectorTest::test)
    	.def_static("new", &ns::RenamedVectorTest::new_);
}


}
