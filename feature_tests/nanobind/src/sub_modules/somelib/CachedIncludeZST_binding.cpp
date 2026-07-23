#include "diplomat_nanobind_common.hpp"


#include "CachedIncludeZST.hpp"

namespace somelib {
void add_CachedIncludeZST_binding(nb::module_ mod) {
    nb::class_<somelib::CachedIncludeZST> st(mod, "CachedIncludeZST");
    maybe_bind_default_init(st);
    st;
}

} 