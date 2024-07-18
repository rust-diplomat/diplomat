#ifndef MyStruct_H
#define MyStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyEnum.h"
#ifdef __cplusplus
namespace capi {
#endif

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
#endif
#include "MyEnum.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

MyStruct MyStruct_new();

uint8_t MyStruct_into_a(MyStruct self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
