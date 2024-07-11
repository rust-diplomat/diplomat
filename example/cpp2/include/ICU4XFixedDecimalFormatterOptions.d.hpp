#ifndef ICU4XFixedDecimalFormatterOptions_D_HPP
#define ICU4XFixedDecimalFormatterOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"

namespace icu4x {
struct ICU4XFixedDecimalFormatterOptions;
class ICU4XFixedDecimalGroupingStrategy;
}


namespace icu4x {
namespace capi {
    typedef struct ICU4XFixedDecimalFormatterOptions {
      icu4x::capi::ICU4XFixedDecimalGroupingStrategy grouping_strategy;
      bool some_other_config;
    } ICU4XFixedDecimalFormatterOptions;
} // namespace capi
} // namespace


namespace icu4x {
struct ICU4XFixedDecimalFormatterOptions {
  icu4x::ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;

  inline static icu4x::ICU4XFixedDecimalFormatterOptions default_();

  inline icu4x::capi::ICU4XFixedDecimalFormatterOptions AsFFI() const;
  inline static icu4x::ICU4XFixedDecimalFormatterOptions FromFFI(icu4x::capi::ICU4XFixedDecimalFormatterOptions c_struct);
};

} // namespace
#endif // ICU4XFixedDecimalFormatterOptions_D_HPP
