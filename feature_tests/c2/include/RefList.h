#ifndef RefList_H
#define RefList_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "RefList.d.h"
#include "RefListParameter.d.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

RefList* RefList_node(const RefListParameter* data);
void RefList_destroy(RefList* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // RefList_H
