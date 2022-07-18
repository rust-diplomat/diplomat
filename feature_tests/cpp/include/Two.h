#ifndef Two_H
#define Two_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Two Two;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void Two_destroy(Two* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
