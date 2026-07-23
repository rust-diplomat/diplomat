#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueIterator.hpp"

namespace somelib::ns {
void add_RenamedOpaqueIterator_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueIterator> opaque(mod, "RenamedOpaqueIterator");
    opaque
        .def("__next__", [](somelib::ns::RenamedOpaqueIterator& self){
                auto next = map_inner(self.next());
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            })
            .def("__iter__", [](nb::handle self) { return self; });
}

} 