#include "diplomat_nanobind_common.hpp"


#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"

namespace somelib {
void add_BorrowedFieldsWithBounds_binding(nb::module_ mod) {
    nb::class_<somelib::BorrowedFieldsWithBounds> st(mod, "BorrowedFieldsWithBounds");
    st
        .def(nb::init<>())
        .def(nb::init<std::u16string_view, std::string_view, std::string_view>(), "field_a"_a.none(),  "field_b"_a.none(),  "field_c"_a.none())
        .def_rw("field_a", &somelib::BorrowedFieldsWithBounds::field_a)
        .def_rw("field_b", &somelib::BorrowedFieldsWithBounds::field_b)
        .def_rw("field_c", &somelib::BorrowedFieldsWithBounds::field_c)
        .def_static("from_foo_and_strings", &somelib::BorrowedFieldsWithBounds::from_foo_and_strings, "foo"_a, "dstr16_x"_a, "utf8_str_z"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>());
}

} 