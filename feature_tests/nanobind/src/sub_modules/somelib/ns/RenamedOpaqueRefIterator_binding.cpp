#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueRefIterator.hpp"

namespace somelib::ns {
void add_RenamedOpaqueRefIterator_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueRefIterator> opaque(mod, "RenamedOpaqueRefIterator");
    opaque
        .def("__next__", [](somelib::ns::RenamedOpaqueRefIterator& self){
                auto next = self.next();
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            }, nb::keep_alive<0, 1>(), nb::rv_policy::reference_internal)
            .def("__iter__", [](nb::handle self) { return self; });
}

} 