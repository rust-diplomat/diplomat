#include "diplomat_nanobind_common.hpp"


#include "BorrowingOptionStruct.hpp"

namespace somelib {
void add_BorrowingOptionStruct_binding(nb::module_ mod) {
    nb::class_<somelib::BorrowingOptionStruct> st(mod, "BorrowingOptionStruct");
    st
        .def(nb::init<>())
        .def(nb::init<std::optional<std::string_view>>(), "a"_a.none())
        .def_rw("a", &somelib::BorrowingOptionStruct::a);
}

} 