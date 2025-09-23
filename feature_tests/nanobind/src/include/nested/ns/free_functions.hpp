#ifndef nested_ns_free_functions_HPP
#define nested_ns_free_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace nested::ns {
namespace capi {
    extern "C" {

    bool namespace_nested_ns_fn(bool x);

    } // extern "C"
} // namespace capi
} // namespace

namespace nested::ns {

inline bool Renamednested_ns_fn(bool x) {
    auto result = nested::ns::capi::namespace_nested_ns_fn(x);
    return result;
}
 

} // namespace
#endif // nested_ns_free_functions_HPP
