#ifndef diplomat_free_functions_H
#define diplomat_free_functions_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"






typedef struct DiplomatCallback_free_callback_holder_f_result { bool is_ok;} DiplomatCallback_free_callback_holder_f_result;

typedef struct DiplomatCallback_free_callback_holder_f {
    const void* data;
    DiplomatCallback_free_callback_holder_f_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_free_callback_holder_f;

int32_t free_func_test(int32_t x);

void free_callback_holder(DiplomatCallback_free_callback_holder_f f_cb_wrap);





#endif // diplomat_free_functions_H
