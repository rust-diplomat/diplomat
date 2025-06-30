#ifndef ImportedStruct_D_H
#define ImportedStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "UnimportedEnum.d.h"




typedef struct ImportedStruct {
  UnimportedEnum foo;
  uint8_t count;
} ImportedStruct;

typedef struct ImportedStruct_option {union { ImportedStruct ok; }; bool is_ok; } ImportedStruct_option;





#endif // ImportedStruct_D_H
