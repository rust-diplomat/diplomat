#ifndef FixedDecimalGroupingStrategy_H
#define FixedDecimalGroupingStrategy_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum FixedDecimalGroupingStrategy {
  FixedDecimalGroupingStrategy_Auto = 0,
  FixedDecimalGroupingStrategy_Never = 1,
  FixedDecimalGroupingStrategy_Always = 2,
  FixedDecimalGroupingStrategy_Min2 = 3,
} FixedDecimalGroupingStrategy;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
