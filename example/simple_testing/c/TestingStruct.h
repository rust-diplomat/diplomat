#ifndef TestingStruct_H
#define TestingStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct TestingStruct {
    int32_t x;
    int32_t y;
} TestingStruct;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void TestingStruct_destroy(TestingStruct* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
