#include "diplomat_nanobind_common.hpp"


#include "Bar.hpp"
#include "BorrowedFields.hpp"


void add_BorrowedFields_binding(nb::module_ mod) {
    nb::class_<BorrowedFields>(mod, "BorrowedFields")
        .def(nb::init<>())
        .def(nb::init<std::u16string_view, std::string_view, std::string_view>(), "a"_a.none(),  "b"_a.none(),  "c"_a.none())
        .def_rw("a", &BorrowedFields::a)
        .def_rw("b", &BorrowedFields::b)
        .def_rw("c", &BorrowedFields::c)
        .def_static("from_bar_and_strings", &BorrowedFields::from_bar_and_strings, "bar"_a, "dstr16"_a, "utf8_str"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>());
}

