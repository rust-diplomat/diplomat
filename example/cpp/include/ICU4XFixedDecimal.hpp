#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimal.h"

class ICU4XFixedDecimal;

/**
 * A destruction policy for using ICU4XFixedDecimal with std::unique_ptr.
 */
struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};

/**
 * See the [Rust documentation for `FixedDecimal`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html) for more information.
 */
class ICU4XFixedDecimal {
 public:

  /**
   * Construct an [`ICU4XFixedDecimal`] from an integer.
   */
  static ICU4XFixedDecimal new_(int32_t v);

  /**
   * Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
   * 
   * See the [Rust documentation for `multiply_pow10`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
   */
  void multiply_pow10(int16_t power);

  /**
   * Format the [`ICU4XFixedDecimal`] as a string.
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  template<typename W> diplomat::result<std::monostate, std::monostate> to_string_to_writeable(W& to) const;

  /**
   * Format the [`ICU4XFixedDecimal`] as a string.
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  diplomat::result<std::string, std::monostate> to_string() const;
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
  ICU4XFixedDecimal() = default;
  ICU4XFixedDecimal(ICU4XFixedDecimal&&) noexcept = default;
  ICU4XFixedDecimal& operator=(ICU4XFixedDecimal&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};


inline ICU4XFixedDecimal ICU4XFixedDecimal::new_(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_new(v));
}
inline void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
template<typename W> inline diplomat::result<std::monostate, std::monostate> ICU4XFixedDecimal::to_string_to_writeable(W& to) const {
  capi::DiplomatWriteable to_writer = diplomat::WriteableTrait<W>::Construct(to);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_to_string(this->inner.get(), &to_writer);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, std::monostate> ICU4XFixedDecimal::to_string() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
