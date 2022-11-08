#ifndef MyEnum_H
#define MyEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyEnum_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void MyEnum_destroy(MyEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // MyEnum_H
