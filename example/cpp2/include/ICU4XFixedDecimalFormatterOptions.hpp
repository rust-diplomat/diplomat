#ifndef ICU4XFixedDecimalFormatterOptions_HPP
#define ICU4XFixedDecimalFormatterOptions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalFormatterOptions.h"

#include "ICU4XFixedDecimalFormatterOptions.d.hpp"


inline ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions::default() {
  auto result = capi::ICU4XFixedDecimalFormatterOptions_default();
  return ICU4XFixedDecimalFormatterOptions::FromFFI(result);
}


#endif // ICU4XFixedDecimalFormatterOptions_HPP
