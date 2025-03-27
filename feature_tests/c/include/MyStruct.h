#ifndef MyStruct_H
#define MyStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyStruct.d.h"






MyStruct MyStruct_new(void);

typedef struct MyStruct_new_fallible_result {union {MyStruct ok; }; bool is_ok;} MyStruct_new_fallible_result;
MyStruct_new_fallible_result MyStruct_new_fallible(uint8_t _a);

uint8_t MyStruct_into_a(MyStruct self);

typedef struct MyStruct_returns_zst_result_result { bool is_ok;} MyStruct_returns_zst_result_result;
MyStruct_returns_zst_result_result MyStruct_returns_zst_result(void);

typedef struct MyStruct_fails_zst_result_result { bool is_ok;} MyStruct_fails_zst_result_result;
MyStruct_fails_zst_result_result MyStruct_fails_zst_result(void);






#endif // MyStruct_H
