#ifndef Unnamespaced_H
#define Unnamespaced_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Unnamespaced Unnamespaced;
#ifdef __cplusplus
} // namespace capi
#endif
#include "AttrEnum.h"
#include "AttrOpaque1.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Unnamespaced* namespace_Unnamespaced_make(AttrEnum _e);

void namespace_Unnamespaced_use_namespaced(const Unnamespaced* self, const AttrOpaque1* _n);
void namespace_Unnamespaced_destroy(Unnamespaced* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
