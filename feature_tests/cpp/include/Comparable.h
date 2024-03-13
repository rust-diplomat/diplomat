#ifndef Comparable_H
#define Comparable_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Comparable Comparable;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Comparable* namespace_Comparable_new(uint8_t int);

int8_t namespace_Comparable_cmp(const Comparable* self, const Comparable* other);
void namespace_Comparable_destroy(Comparable* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
