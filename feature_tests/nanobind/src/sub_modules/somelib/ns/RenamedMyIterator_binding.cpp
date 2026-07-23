#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterator.hpp"

namespace somelib::ns {
void add_RenamedMyIterator_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedMyIterator> opaque(mod, "RenamedMyIterator");
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