#ifndef Unnamespaced_H
#define Unnamespaced_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrEnum.d.h"
#include "AttrEnum.h"
#include "AttrOpaque1.d.h"
#include "AttrOpaque1.h"

#include "Unnamespaced.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



Unnamespaced* namespace_Unnamespaced_make(AttrEnum _e);

void namespace_Unnamespaced_use_namespaced(const Unnamespaced* self, const AttrOpaque1* _n);

void namespace_Unnamespaced_destroy(Unnamespaced* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // Unnamespaced_H
