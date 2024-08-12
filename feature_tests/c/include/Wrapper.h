#ifndef Wrapper_H
#define Wrapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TestingStruct.d.h"

#include "Wrapper.d.h"






typedef struct DiplomatCallback_Wrapper_test_multi_arg_callback_f {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multi_arg_callback_f;

typedef struct DiplomatCallback_Wrapper_test_no_args_h {
    const void* data;
    void (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_no_args_h;

typedef struct DiplomatCallback_Wrapper_test_cb_with_struct_f {
    const void* data;
    int32_t (*run_callback)(const void*, TestingStruct );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_cb_with_struct_f;

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_f {
    const void* data;
    int32_t (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_f;

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_g {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_g;


int32_t Wrapper_test_multi_arg_callback(DiplomatCallback_Wrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

int32_t Wrapper_test_no_args(DiplomatCallback_Wrapper_test_no_args_h h_cb_wrap);

int32_t Wrapper_test_cb_with_struct(DiplomatCallback_Wrapper_test_cb_with_struct_f f_cb_wrap);

int32_t Wrapper_test_multiple_cb_args(DiplomatCallback_Wrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_Wrapper_test_multiple_cb_args_g g_cb_wrap);






#endif // Wrapper_H
