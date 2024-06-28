#ifndef OpaqueIterator_H
#define OpaqueIterator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct OpaqueIterator OpaqueIterator;
#ifdef __cplusplus
} // namespace capi
#endif
#include "AttrOpaque1.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

AttrOpaque1* namespace_OpaqueIterator_next(OpaqueIterator* self);
void namespace_OpaqueIterator_destroy(OpaqueIterator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
