#ifndef ns_diplomat_ns_functions_HPP
#define ns_diplomat_ns_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace


  

inline int32_t ns::Renamedfree_func_test(int32_t x) {
  auto result = ns::capi::free_func_test(x);
  return result;
}

#endif // ns_diplomat_ns_functions_HPP
