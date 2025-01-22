#ifndef CallbackWrapper_H
#define CallbackWrapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CallbackTestingStruct.d.h"

#include "CallbackWrapper.d.h"





typedef struct DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f;
typedef struct DiplomatCallback_CallbackWrapper_test_no_args_h {
    const void* data;
    void (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_no_args_h;
typedef struct DiplomatCallback_CallbackWrapper_test_cb_with_struct_f {
    const void* data;
    int32_t (*run_callback)(const void*, CallbackTestingStruct );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_cb_with_struct_f;
typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f {
    const void* data;
    int32_t (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f;
typedef struct DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g;
typedef struct DiplomatCallback_CallbackWrapper_test_str_cb_arg_f {
    const void* data;
    int32_t (*run_callback)(const void*, DiplomatStringView );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_str_cb_arg_f;

int32_t CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

int32_t CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_h h_cb_wrap);

int32_t CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_f f_cb_wrap);

int32_t CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g g_cb_wrap);

int32_t CallbackWrapper_test_str_cb_arg(DiplomatCallback_CallbackWrapper_test_str_cb_arg_f f_cb_wrap);






#endif // CallbackWrapper_H
