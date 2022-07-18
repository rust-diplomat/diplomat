#ifndef ICU4XFixedDecimalFormatResult_H
#define ICU4XFixedDecimalFormatResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XFixedDecimalFormatResult {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XFixedDecimalFormatResult;
#ifdef __cplusplus
} // namespace capi
#endif
typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XFixedDecimalFormatResult_destroy(ICU4XFixedDecimalFormatResult* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
