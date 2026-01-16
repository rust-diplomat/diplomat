#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterator.hpp"

namespace somelib::ns {
void add_RenamedMyIterator_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedMyIterator_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedMyIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedMyIterator> opaque(mod, "RenamedMyIterator", nb::type_slots(somelib_ns_RenamedMyIterator_slots));
    opaque
        .def("__next__", [](somelib::ns::RenamedMyIterator& self){
                auto next = self.next();
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; });
}

} 