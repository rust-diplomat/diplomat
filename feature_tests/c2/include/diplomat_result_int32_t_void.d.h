#ifndef diplomat_result_int32_t_void_D_H
#define diplomat_result_int32_t_void_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct diplomat_result_int32_t_void {
  union {
    int32_t ok;
  };
  bool is_ok;
} diplomat_result_int32_t_void;




#endif // diplomat_result_int32_t_void_D_H
