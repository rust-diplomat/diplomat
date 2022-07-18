#ifndef ICU4XFixedDecimalFormatOptions_H
#define ICU4XFixedDecimalFormatOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimalGroupingStrategy.h"
#include "ICU4XFixedDecimalSignDisplay.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XFixedDecimalFormatOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    ICU4XFixedDecimalSignDisplay sign_display;
} ICU4XFixedDecimalFormatOptions;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XFixedDecimalGroupingStrategy.h"
#include "ICU4XFixedDecimalSignDisplay.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
