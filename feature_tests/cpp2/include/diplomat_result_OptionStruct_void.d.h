#ifndef diplomat_result_OptionStruct_void_D_H
#define diplomat_result_OptionStruct_void_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionStruct.d.h"

namespace capi {


typedef struct diplomat_result_OptionStruct_void {
  union {
    OptionStruct ok;
  };
  bool is_ok;
} diplomat_result_OptionStruct_void;


} // namespace capi

#endif // diplomat_result_OptionStruct_void_D_H
