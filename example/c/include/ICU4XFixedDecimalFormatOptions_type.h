#ifndef ICU4XFixedDecimalFormatOptions_type_H
#define ICU4XFixedDecimalFormatOptions_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimalGroupingStrategy_type.h"
#include "ICU4XFixedDecimalSignDisplay_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XFixedDecimalFormatOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    ICU4XFixedDecimalSignDisplay sign_display;
} ICU4XFixedDecimalFormatOptions;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimalFormatOptions_type_H
