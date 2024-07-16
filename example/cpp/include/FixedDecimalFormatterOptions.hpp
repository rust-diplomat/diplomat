#ifndef FixedDecimalFormatterOptions_HPP
#define FixedDecimalFormatterOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "FixedDecimalFormatterOptions.h"

#include "FixedDecimalGroupingStrategy.hpp"
struct FixedDecimalFormatterOptions;

struct FixedDecimalFormatterOptions {
 public:
  FixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
  static FixedDecimalFormatterOptions default_();
};


inline FixedDecimalFormatterOptions FixedDecimalFormatterOptions::default_() {
  capi::FixedDecimalFormatterOptions diplomat_raw_struct_out_value = capi::icu4x_FixedDecimalFormatterOptions_default_mv1();
  return FixedDecimalFormatterOptions{ .grouping_strategy = std::move(static_cast<FixedDecimalGroupingStrategy>(diplomat_raw_struct_out_value.grouping_strategy)), .some_other_config = std::move(diplomat_raw_struct_out_value.some_other_config) };
}
#endif
