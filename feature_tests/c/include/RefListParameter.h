#ifndef RefListParameter_H
#define RefListParameter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct RefListParameter RefListParameter;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void RefListParameter_destroy(RefListParameter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
