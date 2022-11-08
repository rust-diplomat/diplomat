#ifndef MyStruct_H
#define MyStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyStruct_type.h"
#include "MyEnum_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

MyStruct MyStruct_new();
void MyStruct_destroy(MyStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // MyStruct_H
