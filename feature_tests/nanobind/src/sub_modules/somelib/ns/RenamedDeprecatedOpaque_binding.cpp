#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedOpaque.hpp"

namespace somelib::ns {
void add_RenamedDeprecatedOpaque_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedDeprecatedOpaque> opaque(mod, "RenamedDeprecatedOpaque", ".. deprecated:: use Foo\n");
}

} 