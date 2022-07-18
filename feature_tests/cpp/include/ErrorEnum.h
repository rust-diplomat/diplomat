#ifndef ErrorEnum_H
#define ErrorEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ErrorEnum {
  ErrorEnum_Foo = 0,
  ErrorEnum_Bar = 1,
} ErrorEnum;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ErrorEnum_destroy(ErrorEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
