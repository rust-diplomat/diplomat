#ifndef diplomat_result_box_ResultOpaque_ErrorEnum_D_H
#define diplomat_result_box_ResultOpaque_ErrorEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ErrorEnum.d.h"
#include "ResultOpaque.d.h"




typedef struct diplomat_result_box_ResultOpaque_ErrorEnum {
  union {
    ResultOpaque* ok;
    ErrorEnum err;
  };
  bool is_ok;
} diplomat_result_box_ResultOpaque_ErrorEnum;




#endif // diplomat_result_box_ResultOpaque_ErrorEnum_D_H
