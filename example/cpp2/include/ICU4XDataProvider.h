#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_void_void.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct diplomat_result_void_void diplomat_result_void_void;


typedef struct ICU4XDataProvider ICU4XDataProvider;



ICU4XDataProvider* ICU4XDataProvider_new_static();
diplomat_result_void_void ICU4XDataProvider_returns_result();
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ICU4XDataProvider_H
