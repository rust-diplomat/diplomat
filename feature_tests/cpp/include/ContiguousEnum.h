#ifndef ContiguousEnum_H
#define ContiguousEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ContiguousEnum {
  ContiguousEnum_C = 0,
  ContiguousEnum_D = 1,
  ContiguousEnum_E = 2,
  ContiguousEnum_F = 3,
} ContiguousEnum;
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
