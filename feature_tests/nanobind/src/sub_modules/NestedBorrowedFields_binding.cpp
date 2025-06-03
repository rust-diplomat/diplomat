#include "diplomat_nanobind_common.hpp"


#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "NestedBorrowedFields.hpp"


void add_NestedBorrowedFields_binding(nb::handle mod) {
    nb::class_<NestedBorrowedFields>(mod, "NestedBorrowedFields")
        .def(nb::init<>())
        .def(nb::init<BorrowedFields, BorrowedFieldsWithBounds, BorrowedFieldsWithBounds>(), "fields"_a.none(),  "bounds"_a.none(),  "bounds2"_a.none())
        .def_rw("fields", &NestedBorrowedFields::fields)
        .def_rw("bounds", &NestedBorrowedFields::bounds)
        .def_rw("bounds2", &NestedBorrowedFields::bounds2)
    	.def_static("from_bar_and_foo_and_strings", &NestedBorrowedFields::from_bar_and_foo_and_strings, "bar"_a, "foo"_a, "dstr16_x"_a, "dstr16_z"_a, "utf8_str_y"_a, "utf8_str_z"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>(), nb::keep_alive<0, 5>(), nb::keep_alive<0, 6>());
}

