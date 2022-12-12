#ifndef RefList_H
#define RefList_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "RefListParameter.d.h"

#include "RefList.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


RefList* RefList_node(const RefListParameter* data);

void RefList_destroy(RefList* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // RefList_H
