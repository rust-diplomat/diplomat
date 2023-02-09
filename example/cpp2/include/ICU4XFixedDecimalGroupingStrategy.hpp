#ifndef ICU4XFixedDecimalGroupingStrategy_HPP
#define ICU4XFixedDecimalGroupingStrategy_HPP

#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.h"


inline ICU4XFixedDecimalGroupingStrategy::ICU4XFixedDecimalGroupingStrategy(ICU4XFixedDecimalGroupingStrategy::Value cpp_value) {
  switch (cpp_value) {
    case Auto:
      value = capi::ICU4XFixedDecimalGroupingStrategy_Auto;
      break;
    case Never:
      value = capi::ICU4XFixedDecimalGroupingStrategy_Never;
      break;
    case Always:
      value = capi::ICU4XFixedDecimalGroupingStrategy_Always;
      break;
    case Min2:
      value = capi::ICU4XFixedDecimalGroupingStrategy_Min2;
      break;
    default:
      abort();
  }
}

inline capi::ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::AsFFI() const {
  return value;
}

inline ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::FromFFI(capi::ICU4XFixedDecimalGroupingStrategy c_enum) {
  return ICU4XFixedDecimalGroupingStrategy(c_enum);
}

#endif // ICU4XFixedDecimalGroupingStrategy_HPP
