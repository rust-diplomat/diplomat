#ifndef RefList_H
#define RefList_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "RefListParameter.d.h"
#include "RefListParameter.h"

#include "RefList.d.h"

namespace capi {


extern "C" {

RefList* RefList_node(const RefListParameter* data);


void RefList_destroy(RefList* self);

} // extern "C"

} // namespace capi

#endif // RefList_H
