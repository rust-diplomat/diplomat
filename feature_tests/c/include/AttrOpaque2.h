#ifndef AttrOpaque2_H
#define AttrOpaque2_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct AttrOpaque2 AttrOpaque2;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void namespace_AttrOpaque2_destroy(AttrOpaque2* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
