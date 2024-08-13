#ifndef MyEnum_D_H
#define MyEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum MyEnum {
  MyEnum_A = -2,
  MyEnum_B = -1,
  MyEnum_C = 0,
  MyEnum_D = 1,
  MyEnum_E = 2,
  MyEnum_F = 3,
} MyEnum;

typedef struct MyEnum_option {union { MyEnum ok; }; bool is_ok; } MyEnum_option;



#endif // MyEnum_D_H
