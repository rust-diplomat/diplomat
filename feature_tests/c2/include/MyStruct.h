#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyStruct.d.h"






MyStruct MyStruct_new();

uint8_t MyStruct_into_a(MyStruct self);

struct MyStruct_returns_zst_result_result { bool is_ok;};
struct MyStruct_returns_zst_result_result MyStruct_returns_zst_result();






#endif // MyStruct_H
