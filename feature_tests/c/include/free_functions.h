#ifndef free_functions_H
#define free_functions_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"






typedef struct DiplomatCallback_diplomat_external_free_callback_holder_f_result { bool is_ok;} DiplomatCallback_diplomat_external_free_callback_holder_f_result;

typedef struct DiplomatCallback_diplomat_external_free_callback_holder_f {
    const void* data;
    DiplomatCallback_diplomat_external_free_callback_holder_f_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_diplomat_external_free_callback_holder_f;

int32_t namespace_free_func_test(int32_t x);

bool namespace_nested_ns_fn(bool x);

void diplomat_external_free_callback_holder(DiplomatCallback_diplomat_external_free_callback_holder_f f_cb_wrap);





#endif // free_functions_H
