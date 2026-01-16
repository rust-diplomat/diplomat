#include "diplomat_nanobind_common.hpp"


#include "BorrowedFieldsReturning.hpp"

namespace somelib {
void add_BorrowedFieldsReturning_binding(nb::module_ mod) {
    nb::class_<somelib::BorrowedFieldsReturning> st(mod, "BorrowedFieldsReturning");
    st
        .def(nb::init<>())
        .def(nb::init<std::string_view>(), "bytes"_a.none())
        .def_rw("bytes", &somelib::BorrowedFieldsReturning::bytes);
}

} 