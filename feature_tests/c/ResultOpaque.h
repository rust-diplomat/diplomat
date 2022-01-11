#ifndef ResultOpaque_H
#define ResultOpaque_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ResultOpaque ResultOpaque;
#include "result_box_ResultOpaque_ErrorEnum.h"
#include "result_box_ResultOpaque_void.h"
#include "result_box_ResultOpaque_ErrorStruct.h"
#include "result_void_box_ResultOpaque.h"
#include "result_ErrorEnum_box_ResultOpaque.h"

result_ffi_result_box_ResultOpaque_ErrorEnum ResultOpaque_new(int32_t i);

result_ffi_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_foo();

result_ffi_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_bar();

result_ffi_result_box_ResultOpaque_void ResultOpaque_new_failing_unit();

result_ffi_result_box_ResultOpaque_ErrorStruct ResultOpaque_new_failing_struct(int32_t i);

result_ffi_result_void_box_ResultOpaque ResultOpaque_new_in_err(int32_t i);

result_ffi_result_ErrorEnum_box_ResultOpaque ResultOpaque_new_in_enum_err(int32_t i);

void ResultOpaque_assert_integer(const ResultOpaque* self, int32_t i);
void ResultOpaque_destroy(ResultOpaque* self);

#ifdef __cplusplus
}
#endif
#endif
