#ifndef FixedDecimalFormatterOptions_D_HPP
#define FixedDecimalFormatterOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "FixedDecimalGroupingStrategy.d.hpp"

namespace icu4x {
struct FixedDecimalFormatterOptions;
class FixedDecimalGroupingStrategy;
}


namespace icu4x {
namespace capi {
    struct FixedDecimalFormatterOptions {
      icu4x::capi::FixedDecimalGroupingStrategy grouping_strategy;
      bool some_other_config;
    };
} // namespace capi
} // namespace


namespace icu4x {
struct FixedDecimalFormatterOptions {
  icu4x::FixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;

  inline static icu4x::FixedDecimalFormatterOptions default_();

  inline icu4x::capi::FixedDecimalFormatterOptions AsFFI() const;
  inline static icu4x::FixedDecimalFormatterOptions FromFFI(icu4x::capi::FixedDecimalFormatterOptions c_struct);
};

} // namespace
#endif // FixedDecimalFormatterOptions_D_HPP
