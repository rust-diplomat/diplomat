#ifndef TraitTestingStruct_D_H
#define TraitTestingStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TraitTestingStruct {
  int32_t x;
  int32_t y;
} TraitTestingStruct;

typedef struct TraitTestingStruct_option {union { TraitTestingStruct ok; }; bool is_ok; } TraitTestingStruct_option;





#endif // TraitTestingStruct_D_H
