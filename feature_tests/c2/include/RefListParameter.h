#ifndef RefListParameter_H
#define RefListParameter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "RefListParameter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


void RefListParameter_destroy(RefListParameter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // RefListParameter_H
