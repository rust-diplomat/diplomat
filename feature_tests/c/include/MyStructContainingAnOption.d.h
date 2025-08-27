#ifndef MyStructContainingAnOption_D_H
#define MyStructContainingAnOption_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DefaultEnum.d.h"
#include "MyStruct.d.h"



typedef struct MyStructContainingAnOption {
  MyStruct_option a;
  DefaultEnum_option b;
} MyStructContainingAnOption;

typedef struct MyStructContainingAnOption_option {union { MyStructContainingAnOption ok; }; bool is_ok; } MyStructContainingAnOption_option;

#endif // MyStructContainingAnOption_D_H
