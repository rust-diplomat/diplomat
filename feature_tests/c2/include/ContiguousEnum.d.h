#ifndef ContiguousEnum_D_H
#define ContiguousEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ContiguousEnum {
  ContiguousEnum_C = 0,
  ContiguousEnum_D = 1,
  ContiguousEnum_E = 2,
  ContiguousEnum_F = 3,
} ContiguousEnum;



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ContiguousEnum_D_H
