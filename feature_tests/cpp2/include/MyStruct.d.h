#ifndef MyStruct_D_H
#define MyStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyEnum.d.h"

namespace capi {


typedef struct MyStruct {
  uint8_t a;
  bool b;
  uint8_t c;
  uint64_t d;
  int32_t e;
  char32_t f;
  MyEnum g;
} MyStruct;



} // namespace capi

#endif // MyStruct_D_H
