#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include "diplomat_runtime.hpp"

namespace capi {
#include "api.h"
}


struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:
  ICU4XDataProvider ICU4XDataProvider_new_static() {
    return ICU4XDataProvider(capi::ICU4XDataProvider_new_static());
  }
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
 private:
  ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};

struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};
class ICU4XFixedDecimal {
 public:
  ICU4XFixedDecimal ICU4XFixedDecimal_new(int32_t v) {
    return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_new(v));
  }
  void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal& self, int16_t power) {
    capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
  }
  void ICU4XFixedDecimal_negate(ICU4XFixedDecimal& self) {
    capi::ICU4XFixedDecimal_negate(this->inner.get());
  }
  std::string ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal& self) {
    std::string diplomat_writeable_string;
    capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
    capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
    return diplomat_writeable_string;
  }
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
 private:
  ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};

struct ICU4XFixedDecimalFormatDeleter {
  void operator()(capi::ICU4XFixedDecimalFormat* l) const noexcept {
    capi::ICU4XFixedDecimalFormat_destroy(l);
  }
};
class ICU4XFixedDecimalFormat {
 public:
  capi::ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat_try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, capi::ICU4XFixedDecimalFormatOptions options) {
    return capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(), provider.AsFFI(), options);
  }
  std::string ICU4XFixedDecimalFormat_format_write(const ICU4XFixedDecimalFormat& self, const ICU4XFixedDecimal& value) {
    std::string diplomat_writeable_string;
    capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
    capi::ICU4XFixedDecimalFormat_format_write(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
    return diplomat_writeable_string;
  }
  inline const capi::ICU4XFixedDecimalFormat* AsFFI() const { return this->inner.get(); }
 private:
  ICU4XFixedDecimalFormat(capi::ICU4XFixedDecimalFormat* i) : inner(i) {}
  std::unique_ptr<capi::ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatDeleter> inner;
};

struct ICU4XFixedDecimalFormatOptionsDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatOptions* l) const noexcept {
    capi::ICU4XFixedDecimalFormatOptions_destroy(l);
  }
};

struct ICU4XFixedDecimalFormatResultDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatResult* l) const noexcept {
    capi::ICU4XFixedDecimalFormatResult_destroy(l);
  }
};

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  ICU4XLocale ICU4XLocale_new(const char* name_data, size_t name_len) {
    return ICU4XLocale(capi::ICU4XLocale_new(name_data, name_len));
  }
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
 private:
  ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};
