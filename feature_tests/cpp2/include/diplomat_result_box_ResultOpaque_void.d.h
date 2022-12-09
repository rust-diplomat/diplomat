#ifndef diplomat_result_box_ResultOpaque_void_D_H
#define diplomat_result_box_ResultOpaque_void_D_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ResultOpaque.d.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct diplomat_result_box_ResultOpaque_void {
  union {
    ResultOpaque* ok;
  };
  bool is_ok;
} diplomat_result_box_ResultOpaque_void;


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // diplomat_result_box_ResultOpaque_void_D_H
