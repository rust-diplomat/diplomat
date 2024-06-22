#ifndef OpaqueIterable_H
#define OpaqueIterable_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct OpaqueIterable OpaqueIterable;
#ifdef __cplusplus
} // namespace capi
#endif
#include "OpaqueIterator.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

OpaqueIterator* namespace_OpaqueIterable_iter(const OpaqueIterable* self);
void namespace_OpaqueIterable_destroy(OpaqueIterable* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
