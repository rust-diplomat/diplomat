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

enum struct ICU4XFixedDecimalGroupingStrategy;


struct ICU4XFixedDecimalFormatterOptions {
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
};


#endif // ICU4XFixedDecimalFormatterOptions_D_HPP
