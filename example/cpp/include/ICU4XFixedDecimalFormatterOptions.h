#ifndef ICU4XFixedDecimalFormatterOptions_H
#define ICU4XFixedDecimalFormatterOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimalGroupingStrategy.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XFixedDecimalFormatterOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    bool some_other_config;
} ICU4XFixedDecimalFormatterOptions;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XFixedDecimalGroupingStrategy.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptions_default();
void ICU4XFixedDecimalFormatterOptions_destroy(ICU4XFixedDecimalFormatterOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
