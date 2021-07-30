#ifndef fixed_decimal_ffi_ICU4XFixedDecimal_H
#define fixed_decimal_ffi_ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
#include "fixed_decimal_ffi_result_void_void.h"

ICU4XFixedDecimal* ICU4XFixedDecimal_new(int32_t v);

void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

fixed_decimal_ffi_result_void_void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);
#ifdef __cplusplus
}
#endif
#endif
