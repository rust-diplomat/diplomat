#ifndef ICU4XFixedDecimalFormat_HPP
#define ICU4XFixedDecimalFormat_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.d.hpp"
#include "ICU4XFixedDecimal.d.hpp"
#include "ICU4XFixedDecimalFormat.d.hpp"
#include "ICU4XFixedDecimalFormat.h"
#include "ICU4XLocale.d.hpp"


inline ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  auto result = capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(),
    provider.AsFFI(),
    options.AsFFI());
  return ICU4XFixedDecimalFormat::FromFFI(result);
}

inline std::string ICU4XFixedDecimalFormat::format_write(const ICU4XFixedDecimal& value) const {
  std::string output;
  capi::DiplomatWriteable writeable = diplomat::WriteableFromString(output);
  capi::ICU4XFixedDecimalFormat_format_write(this->AsFFI(),
    value.AsFFI(),
    &writeable);
}

inline const capi::ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimalFormat*>(this);
}

inline capi::ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimalFormat*>(this);
}

inline const ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::FromFFI(const capi::ICU4XFixedDecimalFormat* ptr) {
  return reinterpret_cast<const ICU4XFixedDecimalFormat*>(ptr);
}

inline ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::FromFFI(capi::ICU4XFixedDecimalFormat* ptr) {
  return reinterpret_cast<ICU4XFixedDecimalFormat*>(ptr);
}

inline ICU4XFixedDecimalFormat::~ICU4XFixedDecimalFormat() {
  capi::ICU4XFixedDecimalFormat_destroy(AsFFI());
}


#endif // ICU4XFixedDecimalFormat_HPP
