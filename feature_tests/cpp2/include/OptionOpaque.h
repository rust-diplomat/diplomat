#ifndef OptionOpaque_H
#define OptionOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "OptionStruct.d.h"
#include "OptionStruct.h"

#include "OptionOpaque.d.h"

namespace capi {


extern "C" {

OptionOpaque* OptionOpaque_new(int32_t i);

OptionOpaque* OptionOpaque_new_none();

struct OptionOpaque_returns_result {union {OptionStruct ok; }; bool is_ok;};
struct OptionOpaque_returns_result OptionOpaque_returns();

struct OptionOpaque_option_isize_result {union {intptr_t ok; }; bool is_ok;};
struct OptionOpaque_option_isize_result OptionOpaque_option_isize(const OptionOpaque* self);

struct OptionOpaque_option_usize_result {union {size_t ok; }; bool is_ok;};
struct OptionOpaque_option_usize_result OptionOpaque_option_usize(const OptionOpaque* self);

struct OptionOpaque_option_i32_result {union {int32_t ok; }; bool is_ok;};
struct OptionOpaque_option_i32_result OptionOpaque_option_i32(const OptionOpaque* self);

struct OptionOpaque_option_u32_result {union {uint32_t ok; }; bool is_ok;};
struct OptionOpaque_option_u32_result OptionOpaque_option_u32(const OptionOpaque* self);

OptionStruct OptionOpaque_new_struct();

OptionStruct OptionOpaque_new_struct_nones();

void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);

bool OptionOpaque_option_opaque_argument(const OptionOpaque* arg);


void OptionOpaque_destroy(OptionOpaque* self);

} // extern "C"

} // namespace capi

#endif // OptionOpaque_H
