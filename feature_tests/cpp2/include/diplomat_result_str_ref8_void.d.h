#ifndef diplomat_result_str_ref8_void_D_H
#define diplomat_result_str_ref8_void_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


namespace capi {


typedef struct diplomat_result_str_ref8_void {
  union {
    DiplomatStringView ok;
  };
  bool is_ok;
} diplomat_result_str_ref8_void;


} // namespace capi

#endif // diplomat_result_str_ref8_void_D_H
