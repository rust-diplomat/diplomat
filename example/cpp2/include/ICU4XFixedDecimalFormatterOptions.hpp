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


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::ICU4XFixedDecimalFormatterOptions icu4x_ICU4XFixedDecimalFormatterOptions_default_mv1();
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::ICU4XFixedDecimalFormatterOptions icu4x::ICU4XFixedDecimalFormatterOptions::default_() {
  auto result = icu4x::capi::icu4x_ICU4XFixedDecimalFormatterOptions_default_mv1();
  return icu4x::ICU4XFixedDecimalFormatterOptions::FromFFI(result);
}


inline icu4x::capi::ICU4XFixedDecimalFormatterOptions icu4x::ICU4XFixedDecimalFormatterOptions::AsFFI() const {
  return icu4x::capi::ICU4XFixedDecimalFormatterOptions {
    .grouping_strategy = grouping_strategy.AsFFI(),
    .some_other_config = some_other_config,
  };
}

inline icu4x::ICU4XFixedDecimalFormatterOptions icu4x::ICU4XFixedDecimalFormatterOptions::FromFFI(icu4x::capi::ICU4XFixedDecimalFormatterOptions c_struct) {
  return icu4x::ICU4XFixedDecimalFormatterOptions {
    .grouping_strategy = icu4x::ICU4XFixedDecimalGroupingStrategy::FromFFI(c_struct.grouping_strategy),
    .some_other_config = c_struct.some_other_config,
  };
}


#endif // ICU4XFixedDecimalFormatterOptions_HPP
