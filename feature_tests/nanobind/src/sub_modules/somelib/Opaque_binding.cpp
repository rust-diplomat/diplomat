#include "diplomat_nanobind_common.hpp"


#include "MyStruct.hpp"
#include "Opaque.hpp"

namespace somelib {
void add_Opaque_binding(nb::module_ mod) {
    PyType_Slot somelib_Opaque_slots[] = {
        {Py_tp_free, (void *)somelib::Opaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Opaque> opaque(mod, "Opaque", nb::type_slots(somelib_Opaque_slots));
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::Opaque::new_))))
        .def("assert_struct", &somelib::Opaque::assert_struct, "s"_a, "See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.\n\nSee the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.\n\nAdditional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)")
        .def_static("cmp", &somelib::Opaque::cmp)
        .def_static("from_str", std::move(maybe_op_unwrap(&somelib::Opaque::from_str)), "input"_a)
        .def("get_debug_str", &somelib::Opaque::get_debug_str)
        .def_static("returns_imported", &somelib::Opaque::returns_imported)
        .def_static("returns_usize", &somelib::Opaque::returns_usize)
        .def_static("try_from_utf8", std::move(maybe_op_unwrap(&somelib::Opaque::try_from_utf8)), "input"_a);
}

} 