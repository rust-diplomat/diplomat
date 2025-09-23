#ifndef SOMELIB_nested_ns_free_functions_HPP
#define SOMELIB_nested_ns_free_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace somelib::nested::ns {
namespace capi {
    extern "C" {

    bool namespace_nested_ns_fn(bool x);

    } // extern "C"
} // namespace capi
} // namespace

namespace somelib::nested::ns {

inline bool Renamednested_ns_fn(bool x) {
    auto result = somelib::nested::ns::capi::namespace_nested_ns_fn(x);
    return result;
}
 

} // namespace
#endif // SOMELIB_nested_ns_free_functions_HPP
