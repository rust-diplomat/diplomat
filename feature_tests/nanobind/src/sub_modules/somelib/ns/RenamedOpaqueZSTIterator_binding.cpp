#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueZSTIterator.hpp"

namespace somelib::ns {
void add_RenamedOpaqueZSTIterator_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueZSTIterator> opaque(mod, "RenamedOpaqueZSTIterator", "Tests for https://github.com/rust-diplomat/diplomat/issues/1050.");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIterator::ctor))))
        .def("__next__", [](somelib::ns::RenamedOpaqueZSTIterator& self){
                auto next = map_inner(self.next());
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; })
        .def("__getitem__", [](somelib::ns::RenamedOpaqueZSTIterator* self, size_t index) {
                auto out = map_inner(self->operator[] (index));
                if (out == nullptr) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }}, "_idx"_a)
        .def("__str__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIterator::stringify)));
}

} 