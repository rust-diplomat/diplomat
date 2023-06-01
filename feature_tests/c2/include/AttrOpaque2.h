#ifndef AttrOpaque2_H
#define AttrOpaque2_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrOpaque2.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


void AttrOpaque2_destroy(AttrOpaque2* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // AttrOpaque2_H
