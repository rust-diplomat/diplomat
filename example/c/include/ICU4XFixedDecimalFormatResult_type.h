#ifndef ICU4XFixedDecimalFormatResult_type_H
#define ICU4XFixedDecimalFormatResult_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XFixedDecimalFormatResult {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XFixedDecimalFormatResult;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimalFormatResult_type_H
