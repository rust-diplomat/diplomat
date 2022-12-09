#ifndef ICU4XFixedDecimalFormat_HPP
#define ICU4XFixedDecimalFormat_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormat.d.hpp"
#include "ICU4XFixedDecimalFormat.h"





inline ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  // TODO
}

inline std::string ICU4XFixedDecimalFormat::format_write(const ICU4XFixedDecimal& value) const {
  // TODO
}

inline const capi::ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimalFormat*>(this);
}
inline capi::ICU4XFixedDecimalFormat* ICU4XFixedDecimalFormat::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimalFormat*>(this);
}
inline ICU4XFixedDecimalFormat::~ICU4XFixedDecimalFormat() {
  capi::ICU4XFixedDecimalFormat_destroy(AsFFI());
}


#endif // ICU4XFixedDecimalFormat_HPP
