#ifndef UnimportedEnum_H
#define UnimportedEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum UnimportedEnum {
  UnimportedEnum_A = 0,
  UnimportedEnum_B = 1,
  UnimportedEnum_C = 2,
} UnimportedEnum;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void UnimportedEnum_destroy(UnimportedEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
