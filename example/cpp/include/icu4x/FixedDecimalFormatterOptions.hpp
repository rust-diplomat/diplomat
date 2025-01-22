#ifndef icu4x_FixedDecimalFormatterOptions_HPP
#define icu4x_FixedDecimalFormatterOptions_HPP

#include "FixedDecimalFormatterOptions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "FixedDecimalGroupingStrategy.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::FixedDecimalFormatterOptions icu4x_FixedDecimalFormatterOptions_default_mv1(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::FixedDecimalFormatterOptions icu4x::FixedDecimalFormatterOptions::default_() {
  auto result = icu4x::capi::icu4x_FixedDecimalFormatterOptions_default_mv1();
  return icu4x::FixedDecimalFormatterOptions::FromFFI(result);
}


inline icu4x::capi::FixedDecimalFormatterOptions icu4x::FixedDecimalFormatterOptions::AsFFI() const {
  return icu4x::capi::FixedDecimalFormatterOptions {
    /* .grouping_strategy = */ grouping_strategy.AsFFI(),
    /* .some_other_config = */ some_other_config,
  };
}

inline icu4x::FixedDecimalFormatterOptions icu4x::FixedDecimalFormatterOptions::FromFFI(icu4x::capi::FixedDecimalFormatterOptions c_struct) {
  return icu4x::FixedDecimalFormatterOptions {
    /* .grouping_strategy = */ icu4x::FixedDecimalGroupingStrategy::FromFFI(c_struct.grouping_strategy),
    /* .some_other_config = */ c_struct.some_other_config,
  };
}


#endif // icu4x_FixedDecimalFormatterOptions_HPP
