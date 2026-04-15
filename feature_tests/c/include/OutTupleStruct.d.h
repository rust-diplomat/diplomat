#ifndef OutTupleStruct_D_H
#define OutTupleStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Opaque.d.h"
#include "PrimitiveStruct.d.h"




typedef struct OutTupleStruct {
  int32_t x;
  int32_t y;
  PrimitiveStruct primitive;
  Opaque* opaque;
} OutTupleStruct;

typedef struct OutTupleStruct_option {union { OutTupleStruct ok; }; bool is_ok; } OutTupleStruct_option;



#endif // OutTupleStruct_D_H
