#ifndef CallbackWrapper_H
#define CallbackWrapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CallbackTestingStruct.d.h"
#include "MyString.d.h"
#include "MyStructContainingAnOption.d.h"
#include "Opaque.d.h"
#include "PrimitiveStruct.d.h"

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
typedef struct DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb {
    const void* data;
    void (*run_callback)(const void*, MyString* );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb;
typedef struct DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f {
    const void* data;
    void (*run_callback)(const void*, DiplomatU8View );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f;
typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t_result { bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_output_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_result_output_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_result_output_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_result_output_t;
typedef struct DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result {union {size_t ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_result_usize_output_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_result_usize_output_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_result_usize_output_t;
typedef struct DiplomatCallback_CallbackWrapper_test_option_output_t_result { bool is_ok;} DiplomatCallback_CallbackWrapper_test_option_output_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_option_output_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_option_output_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_option_output_t;
typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result {union {uint32_t ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t;
typedef struct DiplomatCallback_CallbackWrapper_test_option_opaque_t {
    const void* data;
    const Opaque* (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_option_opaque_t;
typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result {union {size_t ok; size_t err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_diplomat_result_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_diplomat_result_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_diplomat_result_t;
typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t_result {union {const Opaque* ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_result_opaque_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_result_opaque_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_result_opaque_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_result_opaque_t;
typedef struct DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result {union {MyStructContainingAnOption ok; size_t err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_inner_conversion_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_inner_conversion_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_inner_conversion_t;
typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t_result {union {DiplomatStringView ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_str_conversion_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_str_conversion_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_str_conversion_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_str_conversion_t;
typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result {union {DiplomatF64View ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_slice_conversion_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_slice_conversion_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_slice_conversion_t;
typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result {union {DiplomatPrimitiveStructView ok; }; bool is_ok;} DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t;
typedef struct DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result {union { const Opaque* err;}; bool is_ok;} DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result;

typedef struct DiplomatCallback_CallbackWrapper_test_opaque_result_error_t {
    const void* data;
    DiplomatCallback_CallbackWrapper_test_opaque_result_error_t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_CallbackWrapper_test_opaque_result_error_t;

int32_t CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

int32_t CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_h h_cb_wrap);

int32_t CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_f f_cb_wrap);

int32_t CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_g g_cb_wrap);

int32_t CallbackWrapper_test_str_cb_arg(DiplomatCallback_CallbackWrapper_test_str_cb_arg_f f_cb_wrap);

void CallbackWrapper_test_opaque_cb_arg(DiplomatCallback_CallbackWrapper_test_opaque_cb_arg_cb cb_cb_wrap, MyString* a);

void CallbackWrapper_test_slice_cb_arg(DiplomatU8View arg, DiplomatCallback_CallbackWrapper_test_slice_cb_arg_f f_cb_wrap);

void CallbackWrapper_test_result_output(DiplomatCallback_CallbackWrapper_test_result_output_t t_cb_wrap);

void CallbackWrapper_test_result_usize_output(DiplomatCallback_CallbackWrapper_test_result_usize_output_t t_cb_wrap);

void CallbackWrapper_test_option_output(DiplomatCallback_CallbackWrapper_test_option_output_t t_cb_wrap);

void CallbackWrapper_test_diplomat_option_output(DiplomatCallback_CallbackWrapper_test_diplomat_option_output_t t_cb_wrap);

void CallbackWrapper_test_option_opaque(DiplomatCallback_CallbackWrapper_test_option_opaque_t t_cb_wrap, DiplomatWrite* write);

void CallbackWrapper_test_diplomat_result(DiplomatCallback_CallbackWrapper_test_diplomat_result_t t_cb_wrap);

void CallbackWrapper_test_result_opaque(DiplomatCallback_CallbackWrapper_test_result_opaque_t t_cb_wrap, DiplomatWrite* write);

void CallbackWrapper_test_inner_conversion(DiplomatCallback_CallbackWrapper_test_inner_conversion_t t_cb_wrap);

void CallbackWrapper_test_str_conversion(DiplomatCallback_CallbackWrapper_test_str_conversion_t t_cb_wrap);

void CallbackWrapper_test_slice_conversion(DiplomatCallback_CallbackWrapper_test_slice_conversion_t t_cb_wrap);

void CallbackWrapper_test_struct_slice_conversion(DiplomatCallback_CallbackWrapper_test_struct_slice_conversion_t t_cb_wrap);

void CallbackWrapper_test_opaque_result_error(DiplomatCallback_CallbackWrapper_test_opaque_result_error_t t_cb_wrap, DiplomatWrite* write);





#endif // CallbackWrapper_H
