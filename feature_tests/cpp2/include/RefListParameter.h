#ifndef RefListParameter_H
#define RefListParameter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "RefListParameter.d.h"

namespace capi {


extern "C" {


void RefListParameter_destroy(RefListParameter* self);

} // extern "C"

} // namespace capi

#endif // RefListParameter_H
