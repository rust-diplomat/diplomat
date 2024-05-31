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


inline capi::ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::AsFFI() const {
  switch (value) {
    case Auto:
      return capi::ICU4XFixedDecimalGroupingStrategy_Auto;
    case Never:
      return capi::ICU4XFixedDecimalGroupingStrategy_Never;
    case Always:
      return capi::ICU4XFixedDecimalGroupingStrategy_Always;
    case Min2:
      return capi::ICU4XFixedDecimalGroupingStrategy_Min2;
    default:
      abort();
  }
}

inline ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::FromFFI(capi::ICU4XFixedDecimalGroupingStrategy c_enum) {
    switch (c_enum) {
    case capi::ICU4XFixedDecimalGroupingStrategy_Auto:
      return ICU4XFixedDecimalGroupingStrategy::Value::Auto;
    case capi::ICU4XFixedDecimalGroupingStrategy_Never:
      return ICU4XFixedDecimalGroupingStrategy::Value::Never;
    case capi::ICU4XFixedDecimalGroupingStrategy_Always:
      return ICU4XFixedDecimalGroupingStrategy::Value::Always;
    case capi::ICU4XFixedDecimalGroupingStrategy_Min2:
      return ICU4XFixedDecimalGroupingStrategy::Value::Min2;
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalGroupingStrategy_HPP
