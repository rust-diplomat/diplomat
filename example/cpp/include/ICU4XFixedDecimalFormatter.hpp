#ifndef ICU4XFixedDecimalFormatter_HPP
#define ICU4XFixedDecimalFormatter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimalFormatter.h"

class ICU4XLocale;
class ICU4XDataProvider;
struct ICU4XFixedDecimalFormatterOptions;
class ICU4XFixedDecimalFormatter;
class ICU4XFixedDecimal;

/**
 * A destruction policy for using ICU4XFixedDecimalFormatter with std::unique_ptr.
 */
struct ICU4XFixedDecimalFormatterDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatter* l) const noexcept {
    capi::ICU4XFixedDecimalFormatter_destroy(l);
  }
};

/**
 * An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
 * 
 * See the [Rust documentation for `FixedDecimalFormatter`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
 */
class ICU4XFixedDecimalFormatter {
 public:

  /**
   * Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
   * 
   * See the [Rust documentation for `try_new`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XFixedDecimalFormatter, std::monostate> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options);

  /**
   * Formats a [`ICU4XFixedDecimal`] to a string.
   * 
   * See the [Rust documentation for `format`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
   */
  template<typename W> void format_write_to_writeable(const ICU4XFixedDecimal& value, W& write) const;

  /**
   * Formats a [`ICU4XFixedDecimal`] to a string.
   * 
   * See the [Rust documentation for `format`](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
   */
  std::string format_write(const ICU4XFixedDecimal& value) const;
  inline const capi::ICU4XFixedDecimalFormatter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimalFormatter* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XFixedDecimalFormatter(capi::ICU4XFixedDecimalFormatter* i) : inner(i) {}
  ICU4XFixedDecimalFormatter() = default;
  ICU4XFixedDecimalFormatter(ICU4XFixedDecimalFormatter&&) noexcept = default;
  ICU4XFixedDecimalFormatter& operator=(ICU4XFixedDecimalFormatter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XFixedDecimalFormatter, ICU4XFixedDecimalFormatterDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XFixedDecimalFormatterOptions.hpp"
#include "ICU4XFixedDecimal.hpp"

inline diplomat::result<ICU4XFixedDecimalFormatter, std::monostate> ICU4XFixedDecimalFormatter::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options) {
  ICU4XFixedDecimalFormatterOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimalFormatter_try_new(locale.AsFFI(), provider.AsFFI(), capi::ICU4XFixedDecimalFormatterOptions{ .grouping_strategy = static_cast<capi::ICU4XFixedDecimalGroupingStrategy>(diplomat_wrapped_struct_options.grouping_strategy), .some_other_config = diplomat_wrapped_struct_options.some_other_config });
  diplomat::result<ICU4XFixedDecimalFormatter, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XFixedDecimalFormatter>(ICU4XFixedDecimalFormatter(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
template<typename W> inline void ICU4XFixedDecimalFormatter::format_write_to_writeable(const ICU4XFixedDecimal& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  capi::ICU4XFixedDecimalFormatter_format_write(this->inner.get(), value.AsFFI(), &write_writer);
}
inline std::string ICU4XFixedDecimalFormatter::format_write(const ICU4XFixedDecimal& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimalFormatter_format_write(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
#endif
