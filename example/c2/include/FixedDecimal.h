#ifndef FixedDecimal_H
#define FixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "FixedDecimal.d.h"






FixedDecimal* FixedDecimal_new(int32_t v) {
    FixedDecimal* icu4x_FixedDecimal_new_mv1(int32_t v);
    return icu4x_FixedDecimal_new_mv1(v);
}

void FixedDecimal_multiply_pow10(FixedDecimal* self, int16_t power) {
    void icu4x_FixedDecimal_multiply_pow10_mv1(FixedDecimal* self, int16_t power);
    return icu4x_FixedDecimal_multiply_pow10_mv1(self, power);
}

typedef struct FixedDecimal_to_string_result { bool is_ok;} FixedDecimal_to_string_result;
FixedDecimal_to_string_result FixedDecimal_to_string(const FixedDecimal* self, DiplomatWrite* write) {
    FixedDecimal_to_string_result icu4x_FixedDecimal_to_string_mv1(const FixedDecimal* self, DiplomatWrite* write);
    return icu4x_FixedDecimal_to_string_mv1(self, write);
}


void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);





#endif // FixedDecimal_H
