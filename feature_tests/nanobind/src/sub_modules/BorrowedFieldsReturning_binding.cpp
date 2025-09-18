#include "diplomat_nanobind_common.hpp"


#include "BorrowedFieldsReturning.hpp"


void add_BorrowedFieldsReturning_binding(nb::module_ mod) {
    nb::class_<BorrowedFieldsReturning>(mod, "BorrowedFieldsReturning")
        .def(nb::init<>())
        .def(nb::init<std::string_view>(), "bytes"_a.none())
        .def_rw("bytes", &BorrowedFieldsReturning::bytes);
}

