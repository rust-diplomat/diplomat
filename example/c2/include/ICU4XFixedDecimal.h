#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XFixedDecimal.d.h"






ICU4XFixedDecimal* icu4x_ICU4XFixedDecimal_new_mv1(int32_t v);

void icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(ICU4XFixedDecimal* self, int16_t power);

typedef struct icu4x_ICU4XFixedDecimal_to_string_mv1_result { bool is_ok;} icu4x_ICU4XFixedDecimal_to_string_mv1_result;
icu4x_ICU4XFixedDecimal_to_string_mv1_result icu4x_ICU4XFixedDecimal_to_string_mv1(const ICU4XFixedDecimal* self, DiplomatWrite* write);


void icu4x_ICU4XFixedDecimal_destroy_mv1(ICU4XFixedDecimal* self);





#endif // ICU4XFixedDecimal_H
