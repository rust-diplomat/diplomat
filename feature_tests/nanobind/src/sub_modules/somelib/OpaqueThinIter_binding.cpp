#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinIter.hpp"

namespace somelib {
void add_OpaqueThinIter_binding(nb::module_ mod) {
    nb::class_<somelib::OpaqueThinIter> opaque(mod, "OpaqueThinIter");
    opaque
        .def("__next__", [](somelib::OpaqueThinIter& self){
                auto next = self.next();
                if (!next) {
                    throw nb::stop_iteration();
                }
                return next_inner_extractor<decltype(next)>::get(std::move(next));
            }, nb::keep_alive<0, 1>(), nb::rv_policy::reference_internal)
            .def("__iter__", [](nb::handle self) { return self; });
}

} 