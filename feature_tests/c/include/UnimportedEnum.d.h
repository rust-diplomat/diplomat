#ifndef UnimportedEnum_D_H
#define UnimportedEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum UnimportedEnum {
  UnimportedEnum_A = 0,
  UnimportedEnum_B = 1,
  UnimportedEnum_C = 2,
} UnimportedEnum;

typedef struct UnimportedEnum_option {union { UnimportedEnum ok; }; bool is_ok; } UnimportedEnum_option;



#endif // UnimportedEnum_D_H
