#ifndef ns_diplomat_ns_functions_HPP
#define ns_diplomat_ns_functions_HPP

#include "diplomat_ns_functions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {

    int32_t namespace_free_func_test(int32_t x);

    } // extern "C"
} // namespace capi
} // namespace


  

inline int32_t ns::Renamedfree_func_test(int32_t x) {
  auto result = ns::capi::namespace_free_func_test(x);
  return result;
}

#endif // ns_diplomat_ns_functions_HPP
