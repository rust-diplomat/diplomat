#ifndef TestMacroStruct_D_H
#define TestMacroStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TestMacroStruct {
  size_t a;
} TestMacroStruct;

typedef struct TestMacroStruct_option {union { TestMacroStruct ok; }; bool is_ok; } TestMacroStruct_option;



#endif // TestMacroStruct_D_H
