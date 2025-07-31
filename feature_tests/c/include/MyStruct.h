#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyStruct.d.h"






MyStruct MyStruct_new(void);

void MyStruct_takes_mut(MyStruct* self, MyStruct* o);

void MyStruct_takes_const(const MyStruct* self, MyStruct* o);

uint8_t MyStruct_into_a(MyStruct self);

typedef struct MyStruct_returns_zst_result_result { bool is_ok;} MyStruct_returns_zst_result_result;
MyStruct_returns_zst_result_result MyStruct_returns_zst_result(void);

typedef struct MyStruct_fails_zst_result_result { bool is_ok;} MyStruct_fails_zst_result_result;
MyStruct_fails_zst_result_result MyStruct_fails_zst_result(void);





#endif // MyStruct_H
