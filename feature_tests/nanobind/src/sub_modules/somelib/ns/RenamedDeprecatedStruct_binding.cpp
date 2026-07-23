#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedStruct.hpp"

namespace somelib::ns {
void add_RenamedDeprecatedStruct_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedDeprecatedStruct> st(mod, "RenamedDeprecatedStruct", ".. deprecated:: use Foo\n");
    maybe_bind_default_init(st);
    st;
}

} 