#ifndef ICU4XFixedDecimalFormatResult_D_H
#define ICU4XFixedDecimalFormatResult_D_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormat.d.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct ICU4XFixedDecimalFormatResult {
  ICU4XFixedDecimalFormat* fdf;
  bool success;
} ICU4XFixedDecimalFormatResult;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ICU4XFixedDecimalFormatResult_D_H
