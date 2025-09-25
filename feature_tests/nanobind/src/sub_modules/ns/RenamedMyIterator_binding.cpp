#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterator.hpp"


namespace ns{

void add_RenamedMyIterator_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedMyIterator_slots[] = {
        {Py_tp_free, (void *)ns::RenamedMyIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedMyIterator>(mod, "RenamedMyIterator", nb::type_slots(ns_RenamedMyIterator_slots))
        .def("__next__", [](ns::RenamedMyIterator& self){
                auto next = self.next();
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; });
}


}
