#ifndef icu4x_FixedDecimal_H
#define icu4x_FixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "icu4x_FixedDecimal.d.h"






icu4x_FixedDecimal* icu4x_FixedDecimal_new(int32_t v) {
    icu4x_FixedDecimal* icu4x_FixedDecimal_new_mv1(int32_t v);
    return icu4x_FixedDecimal_new_mv1(v);
}

void icu4x_FixedDecimal_multiply_pow10(icu4x_FixedDecimal* self, int16_t power) {
    void icu4x_FixedDecimal_multiply_pow10_mv1(icu4x_FixedDecimal* self, int16_t power);
    return icu4x_FixedDecimal_multiply_pow10_mv1(self, power);
}

typedef struct icu4x_FixedDecimal_to_string_result { bool is_ok;} icu4x_FixedDecimal_to_string_result;
icu4x_FixedDecimal_to_string_result icu4x_FixedDecimal_to_string(const icu4x_FixedDecimal* self, DiplomatWrite* write) {
    icu4x_FixedDecimal_to_string_result icu4x_FixedDecimal_to_string_mv1(const icu4x_FixedDecimal* self, DiplomatWrite* write);
    return icu4x_FixedDecimal_to_string_mv1(self, write);
}


void icu4x_FixedDecimal_destroy(icu4x_FixedDecimal* self) {
    void icu4x_FixedDecimal_destroy_mv1(icu4x_FixedDecimal* self);
    icu4x_FixedDecimal_destroy_mv1(self);
}





#endif // icu4x_FixedDecimal_H
