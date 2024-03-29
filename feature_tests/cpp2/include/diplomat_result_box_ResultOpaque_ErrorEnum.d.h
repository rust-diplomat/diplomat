#ifndef diplomat_result_box_ResultOpaque_ErrorEnum_D_H
#define diplomat_result_box_ResultOpaque_ErrorEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ErrorEnum.d.h"
#include "ResultOpaque.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ResultOpaque_ErrorEnum {
  union {
    ResultOpaque* ok;
    ErrorEnum err;
  };
  bool is_ok;
} diplomat_result_box_ResultOpaque_ErrorEnum;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ResultOpaque_ErrorEnum_D_H
