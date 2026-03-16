#ifndef SOMELIB_ns_RenamedNonMacroMixin_D_HPP
#define SOMELIB_ns_RenamedNonMacroMixin_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {

} // namespace capi
} // namespace


namespace somelib::ns {
/**
 * Diplomat will prepend this whole block to the start of attrs.rs,
 * but we currently cannot do the same for proc_macro (until we hit MSRV >= 1.88).
 * So the workaround is to use the path to the module whenever referring to the imported type (as seen above).
 */
struct RenamedNonMacroMixin {

};

} // namespace
#endif // SOMELIB_ns_RenamedNonMacroMixin_D_HPP
