#ifndef FixedDecimalFormatterOptions_D_H
#define FixedDecimalFormatterOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimalGroupingStrategy.d.h"




typedef struct FixedDecimalFormatterOptions {
  FixedDecimalGroupingStrategy grouping_strategy;
  bool some_other_config;
} FixedDecimalFormatterOptions;

typedef struct FixedDecimalFormatterOptions_option {union { FixedDecimalFormatterOptions ok; }; bool is_ok; } FixedDecimalFormatterOptions_option;



#endif // FixedDecimalFormatterOptions_D_H
