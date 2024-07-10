#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_void_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimal* icu4x_ICU4XFixedDecimal_new_mv1(int32_t v);

void icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(ICU4XFixedDecimal* self, int16_t power);

diplomat_result_void_void icu4x_ICU4XFixedDecimal_to_string_mv1(const ICU4XFixedDecimal* self, DiplomatWrite* to);
void icu4x_ICU4XFixedDecimal_mv1_destroy(ICU4XFixedDecimal* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
