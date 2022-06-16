#ifndef Beta_HPP
#define Beta_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "Beta.h"
}

#include "Alpha.hpp"
struct Beta;

/**
 * A destruction policy for using Beta with std::unique_ptr.
 */
struct BetaDeleter {
  void operator()(capi::Beta* l) const noexcept {
    capi::Beta_destroy(l);
  }
};
struct Beta {
 public:
  Alpha alpha_field;
  static Beta new_(uint32_t x, uint32_t y);
};


inline Beta Beta::new_(uint32_t x, uint32_t y) {
  capi::Beta diplomat_raw_struct_out_value = capi::Beta_new(x, y);
  capi::Alpha diplomat_raw_struct_out_value_alpha_field = diplomat_raw_struct_out_value.alpha_field;
  return Beta{ .alpha_field = std::move(Alpha{ .x = std::move(diplomat_raw_struct_out_value_alpha_field.x), .y = std::move(diplomat_raw_struct_out_value_alpha_field.y) }) };
}
#endif
