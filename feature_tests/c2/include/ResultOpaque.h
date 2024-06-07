#ifndef ResultOpaque_H
#define ResultOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ErrorEnum.d.h"
#include "ErrorEnum.h"
#include "ErrorStruct.d.h"
#include "ErrorStruct.h"

#include "ResultOpaque.d.h"






struct ResultOpaque_new_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;};
struct ResultOpaque_new_result ResultOpaque_new(int32_t i);

struct ResultOpaque_new_failing_foo_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;};
struct ResultOpaque_new_failing_foo_result ResultOpaque_new_failing_foo();

struct ResultOpaque_new_failing_bar_result {union {ResultOpaque* ok; ErrorEnum err;}; bool is_ok;};
struct ResultOpaque_new_failing_bar_result ResultOpaque_new_failing_bar();

struct ResultOpaque_new_failing_unit_result {union {ResultOpaque* ok; }; bool is_ok;};
struct ResultOpaque_new_failing_unit_result ResultOpaque_new_failing_unit();

struct ResultOpaque_new_failing_struct_result {union {ResultOpaque* ok; ErrorStruct err;}; bool is_ok;};
struct ResultOpaque_new_failing_struct_result ResultOpaque_new_failing_struct(int32_t i);

struct ResultOpaque_new_in_err_result {union { ResultOpaque* err;}; bool is_ok;};
struct ResultOpaque_new_in_err_result ResultOpaque_new_in_err(int32_t i);

struct ResultOpaque_new_int_result {union {int32_t ok; }; bool is_ok;};
struct ResultOpaque_new_int_result ResultOpaque_new_int(int32_t i);

struct ResultOpaque_new_in_enum_err_result {union {ErrorEnum ok; ResultOpaque* err;}; bool is_ok;};
struct ResultOpaque_new_in_enum_err_result ResultOpaque_new_in_enum_err(int32_t i);

void ResultOpaque_assert_integer(const ResultOpaque* self, int32_t i);


void ResultOpaque_destroy(ResultOpaque* self);





#endif // ResultOpaque_H
