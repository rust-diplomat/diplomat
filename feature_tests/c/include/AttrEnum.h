#ifndef AttrEnum_H
#define AttrEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum AttrEnum {
  AttrEnum_A = 0,
  AttrEnum_B = 1,
  AttrEnum_C = 2,
} AttrEnum;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void namespace_AttrEnum_destroy(AttrEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
