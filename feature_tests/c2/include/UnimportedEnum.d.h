#ifndef UnimportedEnum_D_H
#define UnimportedEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum UnimportedEnum {
  UnimportedEnum_A = 0,
  UnimportedEnum_B = 1,
  UnimportedEnum_C = 2,
} UnimportedEnum;



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // UnimportedEnum_D_H
