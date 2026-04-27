#ifndef ContainingTuple_D_H
#define ContainingTuple_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TupleStruct.d.h"




typedef struct ContainingTuple {
  TupleStruct inner;
} ContainingTuple;

typedef struct ContainingTuple_option {union { ContainingTuple ok; }; bool is_ok; } ContainingTuple_option;



#endif // ContainingTuple_D_H
