#ifndef decimal_ffi_ICU4XFixedDecimalFormat_HPP
#define decimal_ffi_ICU4XFixedDecimalFormat_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "decimal_ffi_ICU4XFixedDecimalFormat.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
struct ICU4XFixedDecimalFormatOptions;
struct ICU4XFixedDecimalFormatResult;
class ICU4XFixedDecimal;

struct ICU4XFixedDecimalFormatDeleter {
  void operator()(capi::ICU4XFixedDecimalFormat* l) const noexcept {
    capi::ICU4XFixedDecimalFormat_destroy(l);
  }
};
class ICU4XFixedDecimalFormat {
 public:
  static ICU4XFixedDecimalFormatResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);
  std::string format_write(const ICU4XFixedDecimal& value);
  inline const capi::ICU4XFixedDecimalFormat* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimalFormat* AsFFIMut() { return this->inner.get(); }
  ICU4XFixedDecimalFormat(capi::ICU4XFixedDecimalFormat* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatDeleter> inner;
};

#include "locale_ffi_ICU4XLocale.hpp"
#include "data_provider_ffi_ICU4XDataProvider.hpp"
#include "decimal_ffi_ICU4XFixedDecimalFormatOptions.hpp"
#include "decimal_ffi_ICU4XFixedDecimalFormatResult.hpp"
#include "fixed_decimal_ffi_ICU4XFixedDecimal.hpp"

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  ICU4XFixedDecimalFormatOptions diplomat_wrapped_struct_options = options;
  capi::ICU4XFixedDecimalFormatResult diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(), provider.AsFFI(), capi::ICU4XFixedDecimalFormatOptions{ .grouping_strategy = static_cast<capi::ICU4XFixedDecimalGroupingStrategy>(diplomat_wrapped_struct_options.grouping_strategy), .sign_display = static_cast<capi::ICU4XFixedDecimalSignDisplay>(diplomat_wrapped_struct_options.sign_display) });
  auto diplomat_optional_raw_out_value_fdf = diplomat_raw_struct_out_value.fdf;
  std::optional<ICU4XFixedDecimalFormat> diplomat_optional_out_value_fdf;
  if (diplomat_optional_raw_out_value_fdf != nullptr) {
    diplomat_optional_out_value_fdf = ICU4XFixedDecimalFormat(diplomat_optional_raw_out_value_fdf);
  } else {
    diplomat_optional_out_value_fdf = std::nullopt;
  }
  return ICU4XFixedDecimalFormatResult{ .fdf = std::move(diplomat_optional_out_value_fdf), .success = std::move(diplomat_raw_struct_out_value.success) };
}
std::string ICU4XFixedDecimalFormat::format_write(const ICU4XFixedDecimal& value) {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimalFormat_format_write(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
#endif
