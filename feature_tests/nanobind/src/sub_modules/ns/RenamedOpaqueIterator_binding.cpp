#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueIterator.hpp"


namespace ns{

void add_RenamedOpaqueIterator_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedOpaqueIterator_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueIterator>(mod, "RenamedOpaqueIterator", nb::type_slots(ns_RenamedOpaqueIterator_slots))
        .def("__next__", [](ns::RenamedOpaqueIterator& self){
                auto next = self.next();
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; });
}


}
