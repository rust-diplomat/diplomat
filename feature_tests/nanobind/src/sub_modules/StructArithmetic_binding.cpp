#include "diplomat_nanobind_common.hpp"


#include "StructArithmetic.hpp"


void add_StructArithmetic_binding(nb::module_ mod) {
    nb::class_<StructArithmetic>(mod, "StructArithmetic")
        .def_rw("x", &StructArithmetic::x)
        .def_rw("y", &StructArithmetic::y)
        .def_prop_rw_static("ORIGIN", [](nb::handle) -> decltype(StructArithmetic::ORIGIN()) { return StructArithmetic::ORIGIN(); },
                    [](nb::handle, StructArithmetic _new_origin)
                      { StructArithmetic::set_origin(_new_origin); })
        .def(nb::self + nb::self)
        .def(nb::self / nb::self)
        .def(nb::self * nb::self)
        .def("__init__", [](StructArithmetic* self, int32_t x, int32_t y){ *self = StructArithmetic::new_(x, y); }, "x"_a, "y"_a)
        .def(nb::self - nb::self);
}

