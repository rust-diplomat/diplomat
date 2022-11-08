#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimal_type.h"
#include "diplomat_result_void_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimal* ICU4XFixedDecimal_new(int32_t v);

void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

diplomat_result_void_void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimal_H
