#include "diplomat_nanobind_common.hpp"


#include "Float64Vec.hpp"

namespace somelib {
void add_Float64Vec_binding(nb::module_ mod) {
    PyType_Slot somelib_Float64Vec_slots[] = {
        {Py_tp_free, (void *)somelib::Float64Vec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Float64Vec> opaque(mod, "Float64Vec", nb::type_slots(somelib_Float64Vec_slots));
    opaque
        .def_prop_ro("asSlice", &somelib::Float64Vec::as_slice)
        .def("borrow", &somelib::Float64Vec::borrow)
        .def("fill_slice", &somelib::Float64Vec::fill_slice, "v"_a)
        .def("__getitem__", [](somelib::Float64Vec* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "i"_a)
        .def_static("new", &somelib::Float64Vec::new_, "v"_a)
        .def_static("new_bool", &somelib::Float64Vec::new_bool, "v"_a ) // unsupported special method NamedConstructor(Some("bool"))
        .def_static("new_f64_be_bytes", &somelib::Float64Vec::new_f64_be_bytes, "v"_a ) // unsupported special method NamedConstructor(Some("f64BeBytes"))
        .def_static("new_i16", &somelib::Float64Vec::new_i16, "v"_a ) // unsupported special method NamedConstructor(Some("i16"))
        .def_static("new_isize", &somelib::Float64Vec::new_isize, "v"_a ) // unsupported special method NamedConstructor(Some("isize"))
        .def_static("new_u16", &somelib::Float64Vec::new_u16, "v"_a ) // unsupported special method NamedConstructor(Some("u16"))
        .def_static("new_usize", &somelib::Float64Vec::new_usize, "v"_a ) // unsupported special method NamedConstructor(Some("usize"))
        .def("set_value", &somelib::Float64Vec::set_value, "new_slice"_a)
        .def("__str__", &somelib::Float64Vec::to_string);
}

} 