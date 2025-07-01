#ifndef CallbackTestingStruct_D_H
#define CallbackTestingStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CallbackTestingStruct {
  int32_t x;
  int32_t y;
} CallbackTestingStruct;

typedef struct CallbackTestingStruct_option {union { CallbackTestingStruct ok; }; bool is_ok; } CallbackTestingStruct_option;





#endif // CallbackTestingStruct_D_H
