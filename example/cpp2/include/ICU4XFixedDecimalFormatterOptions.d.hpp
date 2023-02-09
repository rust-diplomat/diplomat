#ifndef ICU4XFixedDecimalFormatterOptions_D_HPP
#define ICU4XFixedDecimalFormatterOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalFormatterOptions.d.h"
#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"

class ICU4XFixedDecimalGroupingStrategy;


struct ICU4XFixedDecimalFormatterOptions {
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;

  inline static ICU4XFixedDecimalFormatterOptions default_();

  inline capi::ICU4XFixedDecimalFormatterOptions AsFFI() const;
  inline static ICU4XFixedDecimalFormatterOptions FromFFI(capi::ICU4XFixedDecimalFormatterOptions c_struct);
};


#endif // ICU4XFixedDecimalFormatterOptions_D_HPP
