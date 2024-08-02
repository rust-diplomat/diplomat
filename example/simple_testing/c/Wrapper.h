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
    int32_t (*run_callback)(const void*, int32_t);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multi_arg_callback_f;

DiplomatCallback_Wrapper_test_multi_arg_callback_f* C_create_DiplomatCallback_Wrapper_test_multi_arg_callback_f(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_multiarg_void_callback_f {
    const void* data;
    void (*run_callback)(const void*, int32_t, DiplomatStringView);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiarg_void_callback_f;

DiplomatCallback_Wrapper_test_multiarg_void_callback_f* C_create_DiplomatCallback_Wrapper_test_multiarg_void_callback_f(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_mod_array_g {
    const void* data;
    void (*run_callback)(const void*, DiplomatU8View);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_mod_array_g;

DiplomatCallback_Wrapper_test_mod_array_g* C_create_DiplomatCallback_Wrapper_test_mod_array_g(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_no_args_h {
    const void* data;
    void (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_no_args_h;

DiplomatCallback_Wrapper_test_no_args_h* C_create_DiplomatCallback_Wrapper_test_no_args_h(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_cb_with_struct_f {
    const void* data;
    int32_t (*run_callback)(const void*, TestingStruct);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_cb_with_struct_f;

DiplomatCallback_Wrapper_test_cb_with_struct_f* C_create_DiplomatCallback_Wrapper_test_cb_with_struct_f(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_f {
    const void* data;
    int32_t (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_f;

DiplomatCallback_Wrapper_test_multiple_cb_args_f* C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_f(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_g {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t);
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_g;

DiplomatCallback_Wrapper_test_multiple_cb_args_g* C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_g(const void* callback);


int32_t Wrapper_test_multi_arg_callback(DiplomatCallback_Wrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

void Wrapper_test_multiarg_void_callback(DiplomatCallback_Wrapper_test_multiarg_void_callback_f f_cb_wrap);

void Wrapper_test_mod_array(DiplomatCallback_Wrapper_test_mod_array_g g_cb_wrap);

int32_t Wrapper_test_no_args(DiplomatCallback_Wrapper_test_no_args_h h_cb_wrap);

int32_t Wrapper_test_cb_with_struct(DiplomatCallback_Wrapper_test_cb_with_struct_f f_cb_wrap);

int32_t Wrapper_test_multiple_cb_args(DiplomatCallback_Wrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_Wrapper_test_multiple_cb_args_g g_cb_wrap);






#endif // Wrapper_H
