#ifndef OptionOpaque_H
#define OptionOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BorrowingOptionStruct.d.h"
#include "OptionEnum.d.h"
#include "OptionInputStruct.d.h"
#include "OptionStruct.d.h"

#include "OptionOpaque.d.h"






OptionOpaque* OptionOpaque_new(int32_t i);

OptionOpaque* OptionOpaque_new_none(void);

typedef struct OptionOpaque_returns_result {union {OptionStruct ok; }; bool is_ok;} OptionOpaque_returns_result;
OptionOpaque_returns_result OptionOpaque_returns(void);

typedef struct OptionOpaque_option_isize_result {union {intptr_t ok; }; bool is_ok;} OptionOpaque_option_isize_result;
OptionOpaque_option_isize_result OptionOpaque_option_isize(const OptionOpaque* self);

typedef struct OptionOpaque_option_usize_result {union {size_t ok; }; bool is_ok;} OptionOpaque_option_usize_result;
OptionOpaque_option_usize_result OptionOpaque_option_usize(const OptionOpaque* self);

typedef struct OptionOpaque_option_i32_result {union {int32_t ok; }; bool is_ok;} OptionOpaque_option_i32_result;
OptionOpaque_option_i32_result OptionOpaque_option_i32(const OptionOpaque* self);

typedef struct OptionOpaque_option_u32_result {union {uint32_t ok; }; bool is_ok;} OptionOpaque_option_u32_result;
OptionOpaque_option_u32_result OptionOpaque_option_u32(const OptionOpaque* self);

OptionStruct OptionOpaque_new_struct(void);

OptionStruct OptionOpaque_new_struct_nones(void);

const OptionOpaque* OptionOpaque_returns_none_self(const OptionOpaque* self);

const OptionOpaque* OptionOpaque_returns_some_self(const OptionOpaque* self);

void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);

bool OptionOpaque_option_opaque_argument(const OptionOpaque* arg);

typedef struct OptionOpaque_accepts_option_u8_result {union {uint8_t ok; }; bool is_ok;} OptionOpaque_accepts_option_u8_result;
OptionOpaque_accepts_option_u8_result OptionOpaque_accepts_option_u8(OptionU8 arg, uint8_t sentinel);

typedef struct OptionOpaque_accepts_option_enum_result {union {OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_option_enum_result;
OptionOpaque_accepts_option_enum_result OptionOpaque_accepts_option_enum(OptionEnum_option arg, uint8_t sentinel);

void OptionOpaque_accepts_borrowing_option_struct(BorrowingOptionStruct arg);

typedef struct OptionOpaque_accepts_multiple_option_enum_result {union {OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_multiple_option_enum_result;
OptionOpaque_accepts_multiple_option_enum_result OptionOpaque_accepts_multiple_option_enum(uint8_t sentinel1, OptionEnum_option arg1, OptionEnum_option arg2, OptionEnum_option arg3, uint8_t sentinel2);

typedef struct OptionOpaque_accepts_option_input_struct_result {union {OptionInputStruct ok; }; bool is_ok;} OptionOpaque_accepts_option_input_struct_result;
OptionOpaque_accepts_option_input_struct_result OptionOpaque_accepts_option_input_struct(OptionInputStruct_option arg, uint8_t sentinel);

OptionInputStruct OptionOpaque_returns_option_input_struct(void);

size_t OptionOpaque_accepts_option_str(OptionStringView arg, uint8_t sentinel);

bool OptionOpaque_accepts_option_str_slice(OptionStringsView arg, uint8_t sentinel);

int64_t OptionOpaque_accepts_option_primitive(OptionU32View arg, uint8_t sentinel);

void OptionOpaque_destroy(OptionOpaque* self);





#endif // OptionOpaque_H
