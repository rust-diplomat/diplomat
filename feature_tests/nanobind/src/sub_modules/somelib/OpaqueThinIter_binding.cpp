#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinIter.hpp"

namespace somelib {
void add_OpaqueThinIter_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueThinIter_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueThinIter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueThinIter> opaque(mod, "OpaqueThinIter", nb::type_slots(somelib_OpaqueThinIter_slots));
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