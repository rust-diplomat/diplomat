#ifndef Alpha_HPP
#define Alpha_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "Alpha.h"
}


/**
 * A destruction policy for using Alpha with std::unique_ptr.
 */
struct AlphaDeleter {
  void operator()(capi::Alpha* l) const noexcept {
    capi::Alpha_destroy(l);
  }
};
struct Alpha {
 public:
  uint32_t x;
  uint32_t y;
};


#endif
