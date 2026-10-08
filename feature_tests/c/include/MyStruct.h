#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ContiguousEnum.d.h"
#include "MyEnum.d.h"

#include "MyStruct.d.h"






MyStruct MyStruct_new(void);

void MyStruct_takes_mut(MyStruct* self, MyStruct* o);

void MyStruct_takes_const(const MyStruct* self, MyStruct* o);

uint8_t MyStruct_into_a(MyStruct self);

uint8_t MyStruct_take_ref_ret(const MyStruct* self);

typedef struct MyStruct_returns_zst_result_result { bool is_ok;} MyStruct_returns_zst_result_result;
MyStruct_returns_zst_result_result MyStruct_returns_zst_result(void);

typedef struct MyStruct_fails_zst_result_result { bool is_ok;} MyStruct_fails_zst_result_result;
MyStruct_fails_zst_result_result MyStruct_fails_zst_result(void);

void MyStruct_assert_slice(DiplomatMyStructView slice, MyEnum second_value);

void MyStruct_assert_enum_slice(DiplomatContiguousEnumView slice, ContiguousEnum second_value);





#endif // MyStruct_H
