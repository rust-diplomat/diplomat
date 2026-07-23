#include "diplomat_nanobind_common.hpp"


#include "RefList.hpp"
#include "RefListParameter.hpp"

namespace somelib {
void add_RefList_binding(nb::module_ mod) {
    nb::class_<somelib::RefList> opaque(mod, "RefList");
    opaque
        .def_static("node", std::move(maybe_op_unwrap(&somelib::RefList::node)), "data"_a, nb::keep_alive<0, 1>());
}

} 