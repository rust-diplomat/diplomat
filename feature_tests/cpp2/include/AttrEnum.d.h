#ifndef AttrEnum_D_H
#define AttrEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum AttrEnum {
  AttrEnum_A = 0,
  AttrEnum_B = 1,
  AttrEnum_C = 2,
} AttrEnum;



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // AttrEnum_D_H
