#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueZSTIterator.hpp"

namespace somelib::ns {
void add_RenamedOpaqueZSTIterator_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedOpaqueZSTIterator_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedOpaqueZSTIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedOpaqueZSTIterator> opaque(mod, "RenamedOpaqueZSTIterator", nb::type_slots(somelib_ns_RenamedOpaqueZSTIterator_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIterator::ctor))))
        .def("__next__", [](somelib::ns::RenamedOpaqueZSTIterator& self){
                auto next = wrap_func(self.next());
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; })
        .def("__getitem__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIterator::operator[])), "_idx"_a)
        .def("__str__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIterator::stringify)));
}

} 