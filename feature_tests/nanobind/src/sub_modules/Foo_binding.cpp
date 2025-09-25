#include "diplomat_nanobind_common.hpp"


#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"


void add_Foo_binding(nb::module_ mod) {
    PyType_Slot Foo_slots[] = {
        {Py_tp_free, (void *)Foo::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Foo>(mod, "Foo", nb::type_slots(Foo_slots))
        .def("as_returning", &Foo::as_returning, nb::keep_alive<0, 1>())
        .def_prop_ro("bar", &Foo::get_bar)
        .def_static("extract_from_bounds", &Foo::extract_from_bounds, "bounds"_a, "another_string"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>() ) // unsupported special method NamedConstructor(None)
        .def_static("extract_from_fields", &Foo::extract_from_fields, "fields"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
        .def(nb::new_(&Foo::new_), "x"_a, nb::keep_alive<1, 2>())
        .def_static("new_static", &Foo::new_static, "x"_a ) // unsupported special method NamedConstructor(Some("static"))
    ;
}

