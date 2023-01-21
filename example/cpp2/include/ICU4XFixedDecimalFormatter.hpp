#ifndef ICU4XFixedDecimalFormatter_HPP
#define ICU4XFixedDecimalFormatter_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.d.hpp"
#include "ICU4XFixedDecimal.d.hpp"
#include "ICU4XFixedDecimalFormatter.h"
#include "ICU4XLocale.d.hpp"

#include "ICU4XFixedDecimalFormatter.d.hpp"


inline diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, std::monostate> ICU4XFixedDecimalFormatter::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options) {
  auto result = capi::ICU4XFixedDecimalFormatter_try_new(locale.AsFFI(),
    provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, std::monostate>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimalFormatter>>(std::unique_ptr<ICU4XFixedDecimalFormatter>(ICU4XFixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string ICU4XFixedDecimalFormatter::format_write(const ICU4XFixedDecimal& value) const {
  std::string output;
  capi::DiplomatWriteable writeable = diplomat::WriteableFromString(output);
  auto result = capi::ICU4XFixedDecimalFormatter_format_write(this->AsFFI(),
    value.AsFFI(),
    &writeable);
  return /* TODO: Writeable conversion */;
}

inline const capi::ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimalFormatter*>(this);
}

inline capi::ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimalFormatter*>(this);
}

inline const ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::FromFFI(const capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<const ICU4XFixedDecimalFormatter*>(ptr);
}

inline ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::FromFFI(capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<ICU4XFixedDecimalFormatter*>(ptr);
}

inline ICU4XFixedDecimalFormatter::~ICU4XFixedDecimalFormatter() {
  capi::ICU4XFixedDecimalFormatter_destroy(AsFFI());
}


#endif // ICU4XFixedDecimalFormatter_HPP
