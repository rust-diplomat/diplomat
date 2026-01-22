#include "diplomat_nanobind_common.hpp"


#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"

namespace somelib {
void add_Foo_binding(nb::module_ mod) {
    PyType_Slot somelib_Foo_slots[] = {
        {Py_tp_free, (void *)somelib::Foo::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::Foo> opaque(mod, "Foo", nb::type_slots(somelib_Foo_slots));
    opaque
        .def("as_returning", &somelib::Foo::as_returning, nb::keep_alive<0, 1>())
        .def_prop_ro("bar", &somelib::Foo::get_bar)
        .def_static("extract_from_bounds", &somelib::Foo::extract_from_bounds, "bounds"_a, "another_string"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>() ) // unsupported special method NamedConstructor(None)
        .def_static("extract_from_fields", &somelib::Foo::extract_from_fields, "fields"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
        .def(nb::new_(&somelib::Foo::new_), "x"_a, nb::keep_alive<1, 2>())
        .def_static("new_static", &somelib::Foo::new_static, "x"_a ) // unsupported special method NamedConstructor(Some("static"))
    ;
}

} 