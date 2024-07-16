#ifndef FixedDecimal_H
#define FixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "FixedDecimal.d.h"






FixedDecimal* icu4x_FixedDecimal_new_mv1(int32_t v);

void icu4x_FixedDecimal_multiply_pow10_mv1(FixedDecimal* self, int16_t power);

typedef struct icu4x_FixedDecimal_to_string_mv1_result { bool is_ok;} icu4x_FixedDecimal_to_string_mv1_result;
icu4x_FixedDecimal_to_string_mv1_result icu4x_FixedDecimal_to_string_mv1(const FixedDecimal* self, DiplomatWrite* write);


void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);





#endif // FixedDecimal_H
