#include "diplomat_nanobind_common.hpp"


#include "BorrowedFieldsWithBounds.hpp"


void add_BorrowedFieldsWithBounds_binding(nb::handle mod) {
    nb::class_<BorrowedFieldsWithBounds>(mod, "BorrowedFieldsWithBounds")
        .def(nb::init<>())
        .def(nb::init<std::u16string_view, std::string_view, std::string_view>(), "field_a"_a.none(),  "field_b"_a.none(),  "field_c"_a.none())
        .def_rw("field_a", &BorrowedFieldsWithBounds::field_a)
        .def_rw("field_b", &BorrowedFieldsWithBounds::field_b)
        .def_rw("field_c", &BorrowedFieldsWithBounds::field_c)
    	.def_static("from_foo_and_strings", &BorrowedFieldsWithBounds::from_foo_and_strings, "foo"_a, "dstr16_x"_a, "utf8_str_z"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>());
}

