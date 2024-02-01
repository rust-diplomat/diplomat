#ifndef AttrOpaque1_H
#define AttrOpaque1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct AttrOpaque1 AttrOpaque1;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

AttrOpaque1* namespace_AttrOpaque1_new();

uint8_t namespace_AttrOpaque1_method(const AttrOpaque1* self);

uint8_t renamed_in_c_only(const AttrOpaque1* self);

void namespace_AttrOpaque1_method_disabledcpp(const AttrOpaque1* self);
void AttrOpaque1_destroy(AttrOpaque1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
