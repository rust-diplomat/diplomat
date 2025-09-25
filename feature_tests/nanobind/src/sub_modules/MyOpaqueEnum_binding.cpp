#include "diplomat_nanobind_common.hpp"


#include "MyOpaqueEnum.hpp"


void add_MyOpaqueEnum_binding(nb::module_ mod) {
    PyType_Slot MyOpaqueEnum_slots[] = {
        {Py_tp_free, (void *)MyOpaqueEnum::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<MyOpaqueEnum>(mod, "MyOpaqueEnum", nb::type_slots(MyOpaqueEnum_slots))
        .def_static("new", &MyOpaqueEnum::new_)
        .def("to_string", &MyOpaqueEnum::to_string);
}

