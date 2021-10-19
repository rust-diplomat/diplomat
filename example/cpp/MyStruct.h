#ifndef MyStruct_H
#define MyStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MyStruct {
    uint8_t a;
    bool b;
    uint8_t c;
    uint64_t d;
    int32_t e;
    char f;
} MyStruct;

MyStruct MyStruct_new();
void MyStruct_destroy(MyStruct* self);

#ifdef __cplusplus
}
#endif
#endif
