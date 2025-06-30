#ifndef OptionInputStruct_D_H
#define OptionInputStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionEnum.d.h"




typedef struct OptionInputStruct {
  OptionU8 a;
  OptionChar b;
  OptionEnum_option c;
} OptionInputStruct;

typedef struct OptionInputStruct_option {union { OptionInputStruct ok; }; bool is_ok; } OptionInputStruct_option;





#endif // OptionInputStruct_D_H
