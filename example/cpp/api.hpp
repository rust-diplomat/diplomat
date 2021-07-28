#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


class ICU4XDataProvider;

class ICU4XFixedDecimal;

class ICU4XFixedDecimalFormat;

struct ICU4XFixedDecimalFormatOptions;

struct ICU4XFixedDecimalFormatResult;

enum struct ICU4XFixedDecimalGroupingStrategy : ssize_t {
  Auto = 0,
  Never = 1,
  Always = 2,
  Min2 = 3,
};

enum struct ICU4XFixedDecimalSignDisplay : ssize_t {
  Auto = 0,
  Never = 1,
  Always = 2,
  ExceptZero = 3,
  Negative = 4,
};

class ICU4XLocale;

struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:
  static ICU4XDataProvider new_static();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};

struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};
class ICU4XFixedDecimal {
 public:
  static ICU4XFixedDecimal new_(int32_t v);
  void multiply_pow10(int16_t power);
  void negate();
  std::string to_string();
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};

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

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  static ICU4XLocale new_(const std::string_view name);
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};

struct ICU4XFixedDecimalFormatOptionsDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatOptions* l) const noexcept {
    capi::ICU4XFixedDecimalFormatOptions_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatOptions {
 public:
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  ICU4XFixedDecimalSignDisplay sign_display;
  static ICU4XFixedDecimalFormatOptions default_();
};

struct ICU4XFixedDecimalFormatResultDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatResult* l) const noexcept {
    capi::ICU4XFixedDecimalFormatResult_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatResult {
 public:
  std::optional<ICU4XFixedDecimalFormat> fdf;
  bool success;
};

ICU4XDataProvider ICU4XDataProvider::new_static() {
  return ICU4XDataProvider(capi::ICU4XDataProvider_new_static());
}

ICU4XFixedDecimal ICU4XFixedDecimal::new_(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_new(v));
}
void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
void ICU4XFixedDecimal::negate() {
  capi::ICU4XFixedDecimal_negate(this->inner.get());
}
std::string ICU4XFixedDecimal::to_string() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::fixed_decimal_ffi_result_void_void out_value = capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  ICU4XFixedDecimalFormatOptions diplomat_wrapped_struct_options = options;
  capi::ICU4XFixedDecimalFormatResult diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(), provider.AsFFI(), capi::ICU4XFixedDecimalFormatOptions{ .grouping_strategy = static_cast<ssize_t>(diplomat_wrapped_struct_options.grouping_strategy), .sign_display = static_cast<ssize_t>(diplomat_wrapped_struct_options.sign_display) });
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

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions::default_() {
  capi::ICU4XFixedDecimalFormatOptions diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormatOptions_default();
  return ICU4XFixedDecimalFormatOptions{ .grouping_strategy = std::move(ICU4XFixedDecimalGroupingStrategy{ diplomat_raw_struct_out_value.grouping_strategy }), .sign_display = std::move(ICU4XFixedDecimalSignDisplay{ diplomat_raw_struct_out_value.sign_display }) };
}




ICU4XLocale ICU4XLocale::new_(const std::string_view name) {
  return ICU4XLocale(capi::ICU4XLocale_new(name.data(), name.length()));
}
