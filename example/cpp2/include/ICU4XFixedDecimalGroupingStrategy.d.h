#ifndef ICU4XFixedDecimalGroupingStrategy_D_H
#define ICU4XFixedDecimalGroupingStrategy_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XFixedDecimalGroupingStrategy {
  ICU4XFixedDecimalGroupingStrategy_Auto = 0,
  ICU4XFixedDecimalGroupingStrategy_Never = 1,
  ICU4XFixedDecimalGroupingStrategy_Always = 2,
  ICU4XFixedDecimalGroupingStrategy_Min2 = 3,
} ICU4XFixedDecimalGroupingStrategy;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XFixedDecimalGroupingStrategy_D_H
