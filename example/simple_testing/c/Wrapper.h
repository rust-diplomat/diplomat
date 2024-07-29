#ifndef Wrapper_H
#define Wrapper_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Wrapper {
    bool cant_be_empty;
} Wrapper;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t Wrapper_test_rust_fn(DiplomatCallback* f, int32_t x);

void Wrapper_test_multiarg_callback(DiplomatCallback* f);
void Wrapper_destroy(Wrapper* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
