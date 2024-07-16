#ifndef FixedDecimal_HPP
#define FixedDecimal_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "FixedDecimal.h"

class FixedDecimal;

/**
 * A destruction policy for using FixedDecimal with std::unique_ptr.
 */
struct FixedDecimalDeleter {
  void operator()(capi::FixedDecimal* l) const noexcept {
    capi::icu4x_FixedDecimal_destroy_mv1(l);
  }
};

/**
 * See the [Rust documentation for `FixedDecimal`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html) for more information.
 */
class FixedDecimal {
 public:

  /**
   * Construct an [`FixedDecimal`] from an integer.
   */
  static FixedDecimal new_(int32_t v);

  /**
   * Multiply the [`FixedDecimal`] by a given power of ten.
   * 
   * See the [Rust documentation for `multiply_pow10`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
   */
  void multiply_pow10(int16_t power);

  /**
   * Format the [`FixedDecimal`] as a string.
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  template<typename W> diplomat::result<std::monostate, std::monostate> to_string_to_write(W& to) const;

  /**
   * Format the [`FixedDecimal`] as a string.
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  diplomat::result<std::string, std::monostate> to_string() const;
  inline const capi::FixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::FixedDecimal* AsFFIMut() { return this->inner.get(); }
  inline explicit FixedDecimal(capi::FixedDecimal* i) : inner(i) {}
  FixedDecimal() = default;
  FixedDecimal(FixedDecimal&&) noexcept = default;
  FixedDecimal& operator=(FixedDecimal&& other) noexcept = default;
 private:
  std::unique_ptr<capi::FixedDecimal, FixedDecimalDeleter> inner;
};


inline FixedDecimal FixedDecimal::new_(int32_t v) {
  return FixedDecimal(capi::icu4x_FixedDecimal_new_mv1(v));
}
inline void FixedDecimal::multiply_pow10(int16_t power) {
  capi::icu4x_FixedDecimal_multiply_pow10_mv1(this->inner.get(), power);
}
template<typename W> inline diplomat::result<std::monostate, std::monostate> FixedDecimal::to_string_to_write(W& to) const {
  capi::DiplomatWrite to_writer = diplomat::WriteTrait<W>::Construct(to);
  auto diplomat_result_raw_out_value = capi::icu4x_FixedDecimal_to_string_mv1(this->inner.get(), &to_writer);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, std::monostate> FixedDecimal::to_string() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  auto diplomat_result_raw_out_value = capi::icu4x_FixedDecimal_to_string_mv1(this->inner.get(), &diplomat_write_out);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_write_string));
}
#endif
