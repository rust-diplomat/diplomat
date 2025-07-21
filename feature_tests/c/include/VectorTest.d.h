#ifndef VectorTest_D_H
#define VectorTest_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct VectorTest {
  double test;
} VectorTest;

typedef struct VectorTest_option {union { VectorTest ok; }; bool is_ok; } VectorTest_option;



#endif // VectorTest_D_H
