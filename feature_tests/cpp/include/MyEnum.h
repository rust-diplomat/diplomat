#ifndef MyEnum_H
#define MyEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum MyEnum {
  MyEnum_A = -2,
  MyEnum_B = -1,
  MyEnum_C = 0,
  MyEnum_D = 1,
  MyEnum_E = 2,
  MyEnum_F = 3,
} MyEnum;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int8_t MyEnum_into_value(MyEnum self);

MyEnum MyEnum_get_a();

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
