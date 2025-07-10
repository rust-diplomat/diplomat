#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinIter.hpp"


void add_OpaqueThinIter_binding(nb::handle mod) {
    PyType_Slot OpaqueThinIter_slots[] = {
        {Py_tp_free, (void *)OpaqueThinIter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThinIter>(mod, "OpaqueThinIter", nb::type_slots(OpaqueThinIter_slots))
    	.def("__next__", [](OpaqueThinIter& self){
    			auto next = self.next();
    			if (!next) {
    				throw nb::stop_iteration();
    			}
    			return next_inner_extractor<decltype(next)>::get(std::move(next));
    		}, nb::rv_policy::reference_internal)
    		.def("__iter__", [](nb::handle self) { return self; });
}

