#ifndef ICU4XFixedDecimalFormatterOptions_D_H
#define ICU4XFixedDecimalFormatterOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimalGroupingStrategy.d.h"




typedef struct ICU4XFixedDecimalFormatterOptions {
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
} ICU4XFixedDecimalFormatterOptions;





#endif // ICU4XFixedDecimalFormatterOptions_D_H
