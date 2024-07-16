#ifndef FixedDecimalFormatter_HPP
#define FixedDecimalFormatter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "FixedDecimalFormatter.h"

class Locale;
class DataProvider;
struct FixedDecimalFormatterOptions;
class FixedDecimalFormatter;
class FixedDecimal;

/**
 * A destruction policy for using FixedDecimalFormatter with std::unique_ptr.
 */
struct FixedDecimalFormatterDeleter {
  void operator()(capi::FixedDecimalFormatter* l) const noexcept {
    capi::icu4x_FixedDecimalFormatter_destroy_mv1(l);
  }
};

/**
 * An  Fixed Decimal Format object, capable of formatting a [`FixedDecimal`] as a string.
 * 
 * See the [Rust documentation for `FixedDecimalFormatter`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
 */
class FixedDecimalFormatter {
 public:

  /**
   * Creates a new [`FixedDecimalFormatter`] from locale data.
   * 
   * See the [Rust documentation for `try_new`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
   */
  static diplomat::result<FixedDecimalFormatter, std::monostate> try_new(const Locale& locale, const DataProvider& provider, FixedDecimalFormatterOptions options);

  /**
   * Formats a [`FixedDecimal`] to a string.
   * 
   * See the [Rust documentation for `format`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
   */
  template<typename W> void format_write_to_write(const FixedDecimal& value, W& write) const;

  /**
   * Formats a [`FixedDecimal`] to a string.
   * 
   * See the [Rust documentation for `format`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
   */
  std::string format_write(const FixedDecimal& value) const;
  inline const capi::FixedDecimalFormatter* AsFFI() const { return this->inner.get(); }
  inline capi::FixedDecimalFormatter* AsFFIMut() { return this->inner.get(); }
  inline explicit FixedDecimalFormatter(capi::FixedDecimalFormatter* i) : inner(i) {}
  FixedDecimalFormatter() = default;
  FixedDecimalFormatter(FixedDecimalFormatter&&) noexcept = default;
  FixedDecimalFormatter& operator=(FixedDecimalFormatter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::FixedDecimalFormatter, FixedDecimalFormatterDeleter> inner;
};

#include "Locale.hpp"
#include "DataProvider.hpp"
#include "FixedDecimalFormatterOptions.hpp"
#include "FixedDecimal.hpp"

inline diplomat::result<FixedDecimalFormatter, std::monostate> FixedDecimalFormatter::try_new(const Locale& locale, const DataProvider& provider, FixedDecimalFormatterOptions options) {
  FixedDecimalFormatterOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::icu4x_FixedDecimalFormatter_try_new_mv1(locale.AsFFI(), provider.AsFFI(), capi::FixedDecimalFormatterOptions{ .grouping_strategy = static_cast<capi::FixedDecimalGroupingStrategy>(diplomat_wrapped_struct_options.grouping_strategy), .some_other_config = diplomat_wrapped_struct_options.some_other_config });
  diplomat::result<FixedDecimalFormatter, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<FixedDecimalFormatter>(FixedDecimalFormatter(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
template<typename W> inline void FixedDecimalFormatter::format_write_to_write(const FixedDecimal& value, W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::icu4x_FixedDecimalFormatter_format_write_mv1(this->inner.get(), value.AsFFI(), &write_writer);
}
inline std::string FixedDecimalFormatter::format_write(const FixedDecimal& value) const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::icu4x_FixedDecimalFormatter_format_write_mv1(this->inner.get(), value.AsFFI(), &diplomat_write_out);
  return diplomat_write_string;
}
#endif
