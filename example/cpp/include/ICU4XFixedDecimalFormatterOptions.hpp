#ifndef ICU4XFixedDecimalFormatterOptions_HPP
#define ICU4XFixedDecimalFormatterOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimalFormatterOptions.h"

#include "ICU4XFixedDecimalGroupingStrategy.hpp"
struct ICU4XFixedDecimalFormatterOptions;

struct ICU4XFixedDecimalFormatterOptions {
 public:
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
  static ICU4XFixedDecimalFormatterOptions default_();
};


inline ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions::default_() {
  capi::ICU4XFixedDecimalFormatterOptions diplomat_raw_struct_out_value = capi::icu4x_ICU4XFixedDecimalFormatterOptions_default_mv1();
  return ICU4XFixedDecimalFormatterOptions{ .grouping_strategy = std::move(static_cast<ICU4XFixedDecimalGroupingStrategy>(diplomat_raw_struct_out_value.grouping_strategy)), .some_other_config = std::move(diplomat_raw_struct_out_value.some_other_config) };
}
#endif
