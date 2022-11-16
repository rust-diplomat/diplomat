#ifndef RefList_H
#define RefList_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct RefList RefList;
#ifdef __cplusplus
} // namespace capi
#endif
#include "RefListParameter.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

RefList* RefList_node(const RefListParameter* data);
void RefList_destroy(RefList* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
