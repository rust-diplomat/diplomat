#ifndef nested_ns_diplomat_nested_ns_functions_HPP
#define nested_ns_diplomat_nested_ns_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace


  

inline bool ns::Renamednested_ns_fn(bool x) {
  auto result = nested::ns::capi::nested_ns_fn(x);
  return result;
}

#endif // nested_ns_diplomat_nested_ns_functions_HPP
