#include "diplomat_nanobind_common.hpp"


#include "StructArithmetic.hpp"

namespace somelib {
void add_StructArithmetic_binding(nb::module_ mod) {
    nb::class_<somelib::StructArithmetic> st(mod, "StructArithmetic");
    st
        .def_rw("x", &somelib::StructArithmetic::x)
        .def_rw("y", &somelib::StructArithmetic::y)
        .def_prop_rw_static("ORIGIN", [](nb::handle) -> decltype(somelib::StructArithmetic::ORIGIN()) { return somelib::StructArithmetic::ORIGIN(); },
                    [](nb::handle, somelib::StructArithmetic _new_origin)
                      { somelib::StructArithmetic::set_origin(_new_origin); })
        .def("__init__",[](somelib::StructArithmetic* self, int32_t x, int32_t y){ *self = somelib::StructArithmetic::new_(x, y); }, "x"_a, "y"_a)
        .def("__add__", &somelib::StructArithmetic::operator+, nb::is_operator())
        .def("__truediv__", &somelib::StructArithmetic::operator/, nb::is_operator())
        .def("__mul__", &somelib::StructArithmetic::operator*, nb::is_operator())
        .def("__sub__", &somelib::StructArithmetic::operator-, nb::is_operator());
}

} 