#ifndef SOMELIB_ns_RenamedDeprecatedStruct_D_HPP
#define SOMELIB_ns_RenamedDeprecatedStruct_D_HPP

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
 * \deprecated use Foo
 */
struct [[deprecated("use Foo")]] RenamedDeprecatedStruct {

};

} // namespace
#endif // SOMELIB_ns_RenamedDeprecatedStruct_D_HPP
