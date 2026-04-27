#ifndef TupleStruct_D_H
#define TupleStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyStruct.d.h"
#include "Opaque.d.h"




typedef struct TupleStruct {
  int32_t x;
  int32_t y;
  MyStruct st;
  const Opaque* op;
} TupleStruct;

typedef struct TupleStruct_option {union { TupleStruct ok; }; bool is_ok; } TupleStruct_option;



#endif // TupleStruct_D_H
