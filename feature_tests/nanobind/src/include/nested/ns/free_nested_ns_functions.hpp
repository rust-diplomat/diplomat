#ifndef nested_ns_free_nested_ns_functions_HPP
#define nested_ns_free_nested_ns_functions_HPP

#include "free_nested_ns_functions.d.hpp"

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


inline bool nested::ns::Renamednested_ns_fn(bool x) {
  auto result = nested::ns::capi::namespace_nested_ns_fn(x);
  return result;
}

#endif // nested_ns_free_nested_ns_functions_HPP
