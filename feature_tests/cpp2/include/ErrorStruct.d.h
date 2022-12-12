#ifndef ErrorStruct_D_H
#define ErrorStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ErrorStruct {
  int32_t i;
  int32_t j;
} ErrorStruct;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ErrorStruct_D_H
