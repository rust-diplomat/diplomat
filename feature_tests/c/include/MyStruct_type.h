#ifndef MyStruct_type_H
#define MyStruct_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyEnum_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct MyStruct {
    uint8_t a;
    bool b;
    uint8_t c;
    uint64_t d;
    int32_t e;
    char32_t f;
    MyEnum g;
} MyStruct;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // MyStruct_type_H
