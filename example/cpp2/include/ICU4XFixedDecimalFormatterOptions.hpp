#ifndef ICU4XFixedDecimalFormatterOptions_HPP
#define ICU4XFixedDecimalFormatterOptions_HPP

#include "ICU4XFixedDecimalFormatterOptions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.hpp"


namespace capi {
    extern "C" {
    
    ::capi::ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions_default();
    
    
    } // extern "C"
}
inline ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions::default_() {
  auto result = capi::ICU4XFixedDecimalFormatterOptions_default();
  return ICU4XFixedDecimalFormatterOptions::FromFFI(result);
}


inline ::capi::ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions::AsFFI() const {
  return ::capi::ICU4XFixedDecimalFormatterOptions {
    .grouping_strategy = grouping_strategy.AsFFI(),
    .some_other_config = some_other_config,
  };
}

inline ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions::FromFFI(::capi::ICU4XFixedDecimalFormatterOptions c_struct) {
  return ICU4XFixedDecimalFormatterOptions {
    .grouping_strategy = ICU4XFixedDecimalGroupingStrategy::FromFFI(c_struct.grouping_strategy),
    .some_other_config = c_struct.some_other_config,
  };
}


#endif // ICU4XFixedDecimalFormatterOptions_HPP
