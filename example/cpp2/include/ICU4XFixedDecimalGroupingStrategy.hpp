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


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::ICU4XFixedDecimalGroupingStrategy icu4x::ICU4XFixedDecimalGroupingStrategy::AsFFI() const {
  return static_cast<icu4x::capi::ICU4XFixedDecimalGroupingStrategy>(value);
}

inline icu4x::ICU4XFixedDecimalGroupingStrategy icu4x::ICU4XFixedDecimalGroupingStrategy::FromFFI(icu4x::capi::ICU4XFixedDecimalGroupingStrategy c_enum) {
  switch (c_enum) {
    case icu4x::capi::ICU4XFixedDecimalGroupingStrategy_Auto:
    case icu4x::capi::ICU4XFixedDecimalGroupingStrategy_Never:
    case icu4x::capi::ICU4XFixedDecimalGroupingStrategy_Always:
    case icu4x::capi::ICU4XFixedDecimalGroupingStrategy_Min2:
      return static_cast<icu4x::ICU4XFixedDecimalGroupingStrategy::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalGroupingStrategy_HPP
