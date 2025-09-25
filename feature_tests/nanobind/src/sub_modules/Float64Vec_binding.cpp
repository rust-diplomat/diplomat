#include "diplomat_nanobind_common.hpp"


#include "Float64Vec.hpp"


void add_Float64Vec_binding(nb::module_ mod) {
    PyType_Slot Float64Vec_slots[] = {
        {Py_tp_free, (void *)Float64Vec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Float64Vec>(mod, "Float64Vec", nb::type_slots(Float64Vec_slots))
        .def_prop_ro("asSlice", &Float64Vec::as_slice)
        .def("borrow", &Float64Vec::borrow)
        .def("fill_slice", &Float64Vec::fill_slice, "v"_a)
        .def("__getitem__", [](Float64Vec* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "i"_a)
        .def_static("new", &Float64Vec::new_, "v"_a)
        .def_static("new_bool", &Float64Vec::new_bool, "v"_a ) // unsupported special method NamedConstructor(Some("bool"))
        .def_static("new_f64_be_bytes", &Float64Vec::new_f64_be_bytes, "v"_a ) // unsupported special method NamedConstructor(Some("f64BeBytes"))
        .def_static("new_i16", &Float64Vec::new_i16, "v"_a ) // unsupported special method NamedConstructor(Some("i16"))
        .def_static("new_isize", &Float64Vec::new_isize, "v"_a ) // unsupported special method NamedConstructor(Some("isize"))
        .def_static("new_u16", &Float64Vec::new_u16, "v"_a ) // unsupported special method NamedConstructor(Some("u16"))
        .def_static("new_usize", &Float64Vec::new_usize, "v"_a ) // unsupported special method NamedConstructor(Some("usize"))
        .def("set_value", &Float64Vec::set_value, "new_slice"_a)
        .def("__str__", &Float64Vec::to_string);
}

