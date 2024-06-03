#ifndef diplomat_result_void_box_ResultOpaque_D_H
#define diplomat_result_void_box_ResultOpaque_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ResultOpaque.d.h"

namespace capi {


typedef struct diplomat_result_void_box_ResultOpaque {
  union {
    ResultOpaque* err;
  };
  bool is_ok;
} diplomat_result_void_box_ResultOpaque;


} // namespace capi

#endif // diplomat_result_void_box_ResultOpaque_D_H
