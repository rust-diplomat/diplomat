#ifndef FixedDecimal_H
#define FixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct FixedDecimal FixedDecimal;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_void_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

FixedDecimal* icu4x_FixedDecimal_new_mv1(int32_t v);

void icu4x_FixedDecimal_multiply_pow10_mv1(FixedDecimal* self, int16_t power);

diplomat_result_void_void icu4x_FixedDecimal_to_string_mv1(const FixedDecimal* self, DiplomatWrite* to);
void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
