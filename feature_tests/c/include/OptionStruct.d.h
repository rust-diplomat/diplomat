#ifndef OptionStruct_D_H
#define OptionStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionOpaque.d.h"
#include "OptionOpaqueChar.d.h"




typedef struct OptionStruct {
  OptionOpaque* a;
  OptionOpaqueChar* b;
  uint32_t c;
  OptionOpaque* d;
} OptionStruct;

typedef struct OptionStruct_option {union { OptionStruct ok; }; bool is_ok; } OptionStruct_option;



#endif // OptionStruct_D_H
