#ifndef MyStruct_D_H
#define MyStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyEnum.d.h"




typedef struct MyStruct {
  uint8_t a;
  bool b;
  uint8_t c;
  uint64_t d;
  int32_t e;
  char32_t f;
  MyEnum g;
} MyStruct;

typedef struct MyStruct_option {union { MyStruct ok; }; bool is_ok; } MyStruct_option;
typedef struct DiplomatMyStructView {
  const MyStruct* data;
  size_t len;
} DiplomatMyStructView;

typedef struct DiplomatMyStructViewMut {
  MyStruct* data;
  size_t len;
} DiplomatMyStructViewMut;




#endif // MyStruct_D_H
