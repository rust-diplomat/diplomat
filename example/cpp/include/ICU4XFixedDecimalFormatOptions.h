#ifndef ICU4XFixedDecimalFormatOptions_H
#define ICU4XFixedDecimalFormatOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormatOptions_type.h"
#include "ICU4XFixedDecimalGroupingStrategy_type.h"
#include "ICU4XFixedDecimalSignDisplay_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimalFormatOptions_H
