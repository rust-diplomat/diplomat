#ifndef ResultOpaque_H
#define ResultOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "diplomat_result_ErrorEnum_box_ResultOpaque.d.h"
#include "diplomat_result_box_ResultOpaque_ErrorEnum.d.h"
#include "diplomat_result_box_ResultOpaque_ErrorStruct.d.h"
#include "diplomat_result_box_ResultOpaque_void.d.h"
#include "diplomat_result_int32_t_void.d.h"
#include "diplomat_result_void_box_ResultOpaque.d.h"

#include "ResultOpaque.d.h"






diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new(int32_t i);

diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_foo();

diplomat_result_box_ResultOpaque_ErrorEnum ResultOpaque_new_failing_bar();

diplomat_result_box_ResultOpaque_void ResultOpaque_new_failing_unit();

diplomat_result_box_ResultOpaque_ErrorStruct ResultOpaque_new_failing_struct(int32_t i);

diplomat_result_void_box_ResultOpaque ResultOpaque_new_in_err(int32_t i);

diplomat_result_int32_t_void ResultOpaque_new_int(int32_t i);

diplomat_result_ErrorEnum_box_ResultOpaque ResultOpaque_new_in_enum_err(int32_t i);

void ResultOpaque_assert_integer(const ResultOpaque* self, int32_t i);


void ResultOpaque_destroy(ResultOpaque* self);





#endif // ResultOpaque_H
