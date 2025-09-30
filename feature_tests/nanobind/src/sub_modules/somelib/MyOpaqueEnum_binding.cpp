#include "diplomat_nanobind_common.hpp"


#include "MyOpaqueEnum.hpp"

namespace somelib {
void add_MyOpaqueEnum_binding(nb::module_ mod) {
    PyType_Slot somelib_MyOpaqueEnum_slots[] = {
        {Py_tp_free, (void *)somelib::MyOpaqueEnum::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::MyOpaqueEnum>(mod, "MyOpaqueEnum", nb::type_slots(somelib_MyOpaqueEnum_slots))
        .def_static("new", &somelib::MyOpaqueEnum::new_)
        .def("to_string", &somelib::MyOpaqueEnum::to_string);
}

} 