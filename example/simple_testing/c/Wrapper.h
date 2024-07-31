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
#include "diplomat_callback_structs.h"
#include "TestingStruct.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

int32_t Wrapper_test_multi_arg_callback(DiplomatCallback_int32_t f, int32_t x);

void Wrapper_test_multiarg_void_callback(DiplomatCallback_void f);

void Wrapper_test_mod_array(DiplomatCallback_void g);

int32_t Wrapper_test_no_args(DiplomatCallback_void h);

int32_t Wrapper_test_cb_with_struct(DiplomatCallback_int32_t f);

int32_t Wrapper_test_multiple_cb_args(DiplomatCallback_int32_t f, DiplomatCallback_int32_t g);
void Wrapper_destroy(Wrapper* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
