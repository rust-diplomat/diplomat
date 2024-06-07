#ifndef diplomat_result_box_ResultOpaque_ErrorStruct_D_H
#define diplomat_result_box_ResultOpaque_ErrorStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ErrorStruct.d.h"
#include "ResultOpaque.d.h"




typedef struct diplomat_result_box_ResultOpaque_ErrorStruct {
  union {
    ResultOpaque* ok;
    ErrorStruct err;
  };
  bool is_ok;
} diplomat_result_box_ResultOpaque_ErrorStruct;




#endif // diplomat_result_box_ResultOpaque_ErrorStruct_D_H
