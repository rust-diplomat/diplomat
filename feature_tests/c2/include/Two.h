#ifndef Two_H
#define Two_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

void Two_destroy(Two* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // Two_H
