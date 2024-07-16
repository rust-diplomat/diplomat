#ifndef DataProvider_H
#define DataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct DataProvider DataProvider;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_void_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

DataProvider* icu4x_DataProvider_new_static_mv1();

diplomat_result_void_void icu4x_DataProvider_returns_result_mv1();
void icu4x_DataProvider_destroy_mv1(DataProvider* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
