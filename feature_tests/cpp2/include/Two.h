#ifndef Two_H
#define Two_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Two.d.h"

namespace capi {


extern "C" {


void Two_destroy(Two* self);

} // extern "C"

} // namespace capi

#endif // Two_H
