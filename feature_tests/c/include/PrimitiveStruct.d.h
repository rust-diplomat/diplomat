#ifndef PrimitiveStruct_D_H
#define PrimitiveStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct PrimitiveStruct {
  float x;
  bool a;
  char32_t b;
  int64_t c;
  intptr_t d;
  uint8_t e;
} PrimitiveStruct;

typedef struct PrimitiveStruct_option {union { PrimitiveStruct ok; }; bool is_ok; } PrimitiveStruct_option;
typedef struct DiplomatPrimitiveStructView {
  const PrimitiveStruct* data;
  size_t len;
} DiplomatPrimitiveStructView;

typedef struct DiplomatPrimitiveStructViewMut {
  PrimitiveStruct* data;
  size_t len;
} DiplomatPrimitiveStructViewMut;




#endif // PrimitiveStruct_D_H
