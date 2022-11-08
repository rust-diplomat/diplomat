#ifndef UnimportedEnum_H
#define UnimportedEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "UnimportedEnum_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void UnimportedEnum_destroy(UnimportedEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // UnimportedEnum_H
