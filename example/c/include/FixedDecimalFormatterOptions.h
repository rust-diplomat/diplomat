#ifndef FixedDecimalFormatterOptions_H
#define FixedDecimalFormatterOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimalGroupingStrategy.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct FixedDecimalFormatterOptions {
    FixedDecimalGroupingStrategy grouping_strategy;
    bool some_other_config;
} FixedDecimalFormatterOptions;
#ifdef __cplusplus
} // namespace capi
#endif
#include "FixedDecimalGroupingStrategy.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

FixedDecimalFormatterOptions icu4x_FixedDecimalFormatterOptions_default_mv1();
void icu4x_FixedDecimalFormatterOptions_destroy_mv1(FixedDecimalFormatterOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
