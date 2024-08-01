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
#include "TestingStruct.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

typedef struct DiplomatCallback_Wrapper_test_multi_arg_callback_f {
	const void* data;
	int32_t(*run_callback)(const void*, int32_t);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multi_arg_callback_f;

DiplomatCallback_Wrapper_test_multi_arg_callback_f* C_create_DC_Wrapper_test_multi_arg_callback_f(const void* callback);

int32_t Wrapper_test_multi_arg_callback(DiplomatCallback_Wrapper_test_multi_arg_callback_f f, int32_t x);

typedef struct DiplomatCallback_Wrapper_test_multiarg_void_callback_f {
	const void* data;
	void(*run_callback)(const void*, int32_t, DiplomatStringView);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiarg_void_callback_f;

DiplomatCallback_Wrapper_test_multiarg_void_callback_f* C_create_DC_Wrapper_test_multiarg_void_callback_f(const void* callback);

void Wrapper_test_multiarg_void_callback(DiplomatCallback_Wrapper_test_multiarg_void_callback_f f);

typedef struct DiplomatCallback_Wrapper_test_mod_array_g {
	const void* data;
	void(*run_callback)(const void*, DiplomatU8View);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_mod_array_g;

DiplomatCallback_Wrapper_test_mod_array_g* C_create_DC_Wrapper_test_mod_array_g(const void* callback);

void Wrapper_test_mod_array(DiplomatCallback_Wrapper_test_mod_array_g g);

typedef struct DiplomatCallback_Wrapper_test_no_args_h {
	const void* data;
	void(*run_callback)(const void*);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_no_args_h;

DiplomatCallback_Wrapper_test_no_args_h* C_create_DC_Wrapper_test_no_args_h(const void* callback);

int32_t Wrapper_test_no_args(DiplomatCallback_Wrapper_test_no_args_h h);

typedef struct DiplomatCallback_Wrapper_test_cb_with_struct_f {
	const void* data;
	int32_t(*run_callback)(const void*, TestingStruct);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_cb_with_struct_f;

DiplomatCallback_Wrapper_test_cb_with_struct_f* C_create_DC_Wrapper_test_cb_with_struct_f(const void* callback);

int32_t Wrapper_test_cb_with_struct(DiplomatCallback_Wrapper_test_cb_with_struct_f f);

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_f {
	const void* data;
	int32_t(*run_callback)(const void*);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_f;

DiplomatCallback_Wrapper_test_multiple_cb_args_f* C_create_DC_Wrapper_test_multiple_cb_args_f(const void* callback);

typedef struct DiplomatCallback_Wrapper_test_multiple_cb_args_g {
	const void* data;
	int32_t(*run_callback)(const void*, int32_t);
	void (*destructor)(const void*);
} DiplomatCallback_Wrapper_test_multiple_cb_args_g;

DiplomatCallback_Wrapper_test_multiple_cb_args_g* C_create_DC_Wrapper_test_multiple_cb_args_g(const void* callback);

int32_t Wrapper_test_multiple_cb_args(DiplomatCallback_Wrapper_test_multiple_cb_args_f f, DiplomatCallback_Wrapper_test_multiple_cb_args_g g);
void Wrapper_destroy(Wrapper* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
