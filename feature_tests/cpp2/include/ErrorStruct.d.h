#ifndef ErrorStruct_D_H
#define ErrorStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


namespace capi {


typedef struct ErrorStruct {
  int32_t i;
  int32_t j;
} ErrorStruct;



} // namespace capi

#endif // ErrorStruct_D_H
