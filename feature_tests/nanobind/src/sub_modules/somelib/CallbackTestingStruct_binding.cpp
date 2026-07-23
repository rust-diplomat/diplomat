#include "diplomat_nanobind_common.hpp"


#include "CallbackTestingStruct.hpp"

namespace somelib {
void add_CallbackTestingStruct_binding(nb::module_ mod) {
    nb::class_<somelib::CallbackTestingStruct> st(mod, "CallbackTestingStruct");
    maybe_bind_default_init(st);
    st
        .def(nb::init<int32_t, int32_t>(), "x"_a.none(),  "y"_a.none())
        .def_rw("x", &somelib::CallbackTestingStruct::x)
        .def_rw("y", &somelib::CallbackTestingStruct::y);
}

} 