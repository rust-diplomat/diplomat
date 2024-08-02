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
    int32_t (*run_callback)(int32_t(*cb_pointer)(int32_t), int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multi_arg_callback_f;

int32_t run_create_DiplomatCallback_Wrapper_test_multi_arg_callback_f_callback(int32_t(*cb_pointer)(int32_t), int32_t arg0 ) {
    return cb_pointer(arg0);
}

DiplomatCallback_Wrapper_test_multi_arg_callback_f C_create_DiplomatCallback_Wrapper_test_multi_arg_callback_f(int32_t(*cb_pointer)(int32_t)) {
    DiplomatCallback_Wrapper_test_multi_arg_callback_f ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_multi_arg_callback_f_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_multiarg_void_callback_f {
    const void* data;
    void (*run_callback)(void(*cb_pointer)(int32_t, DiplomatStringView), int32_t, DiplomatStringView );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiarg_void_callback_f;

void run_create_DiplomatCallback_Wrapper_test_multiarg_void_callback_f_callback(void(*cb_pointer)(int32_t, DiplomatStringView), int32_t arg0, DiplomatStringView arg1 ) {
    return cb_pointer(arg0, arg1);
}

DiplomatCallback_Wrapper_test_multiarg_void_callback_f C_create_DiplomatCallback_Wrapper_test_multiarg_void_callback_f(void(*cb_pointer)(int32_t, DiplomatStringView)) {
    DiplomatCallback_Wrapper_test_multiarg_void_callback_f ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_multiarg_void_callback_f_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_mod_array_g {
    const void* data;
    void (*run_callback)(void(*cb_pointer)(DiplomatU8View), DiplomatU8View );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_mod_array_g;

void run_create_DiplomatCallback_Wrapper_test_mod_array_g_callback(void(*cb_pointer)(DiplomatU8View), DiplomatU8View arg0 ) {
    return cb_pointer(arg0);
}

DiplomatCallback_Wrapper_test_mod_array_g C_create_DiplomatCallback_Wrapper_test_mod_array_g(void(*cb_pointer)(DiplomatU8View)) {
    DiplomatCallback_Wrapper_test_mod_array_g ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_mod_array_g_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_no_args_h {
    const void* data;
    void (*run_callback)(void(*cb_pointer)());
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_no_args_h;

void run_create_DiplomatCallback_Wrapper_test_no_args_h_callback(void(*cb_pointer)()) {
    return cb_pointer();
}

DiplomatCallback_Wrapper_test_no_args_h C_create_DiplomatCallback_Wrapper_test_no_args_h(void(*cb_pointer)()) {
    DiplomatCallback_Wrapper_test_no_args_h ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_no_args_h_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_cb_with_struct_f {
    const void* data;
    int32_t (*run_callback)(int32_t(*cb_pointer)(TestingStruct), TestingStruct );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_cb_with_struct_f;

int32_t run_create_DiplomatCallback_Wrapper_test_cb_with_struct_f_callback(int32_t(*cb_pointer)(TestingStruct), TestingStruct arg0 ) {
    return cb_pointer(arg0);
}

DiplomatCallback_Wrapper_test_cb_with_struct_f C_create_DiplomatCallback_Wrapper_test_cb_with_struct_f(int32_t(*cb_pointer)(TestingStruct)) {
    DiplomatCallback_Wrapper_test_cb_with_struct_f ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_cb_with_struct_f_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_f {
    const void* data;
    int32_t (*run_callback)(int32_t(*cb_pointer)());
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_f;

int32_t run_create_DiplomatCallback_Wrapper_test_multiple_cb_args_f_callback(int32_t(*cb_pointer)()) {
    return cb_pointer();
}

DiplomatCallback_Wrapper_test_multiple_cb_args_f C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_f(int32_t(*cb_pointer)()) {
    DiplomatCallback_Wrapper_test_multiple_cb_args_f ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_multiple_cb_args_f_callback,
        // no destructor
    };
    return ret;
}

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_g {
    const void* data;
    int32_t (*run_callback)(int32_t(*cb_pointer)(int32_t), int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_g;

int32_t run_create_DiplomatCallback_Wrapper_test_multiple_cb_args_g_callback(int32_t(*cb_pointer)(int32_t), int32_t arg0 ) {
    return cb_pointer(arg0);
}

DiplomatCallback_Wrapper_test_multiple_cb_args_g C_create_DiplomatCallback_Wrapper_test_multiple_cb_args_g(int32_t(*cb_pointer)(int32_t)) {
    DiplomatCallback_Wrapper_test_multiple_cb_args_g ret = {
        cb_pointer,
        run_create_DiplomatCallback_Wrapper_test_multiple_cb_args_g_callback,
        // no destructor
    };
    return ret;
}


int32_t Wrapper_test_multi_arg_callback(DiplomatCallback_Wrapper_test_multi_arg_callback_f f_cb_wrap, int32_t x);

void Wrapper_test_multiarg_void_callback(DiplomatCallback_Wrapper_test_multiarg_void_callback_f f_cb_wrap);

void Wrapper_test_mod_array(DiplomatCallback_Wrapper_test_mod_array_g g_cb_wrap);

int32_t Wrapper_test_no_args(DiplomatCallback_Wrapper_test_no_args_h h_cb_wrap);

int32_t Wrapper_test_cb_with_struct(DiplomatCallback_Wrapper_test_cb_with_struct_f f_cb_wrap);

int32_t Wrapper_test_multiple_cb_args(DiplomatCallback_Wrapper_test_multiple_cb_args_f f_cb_wrap, DiplomatCallback_Wrapper_test_multiple_cb_args_g g_cb_wrap);






#endif // Wrapper_H
