#ifndef icu4x_FixedDecimalFormatterOptions_D_H
#define icu4x_FixedDecimalFormatterOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "icu4x_FixedDecimalGroupingStrategy.d.h"




typedef struct icu4x_FixedDecimalFormatterOptions {
  icu4x_FixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
} icu4x_FixedDecimalFormatterOptions;





#endif // icu4x_FixedDecimalFormatterOptions_D_H
