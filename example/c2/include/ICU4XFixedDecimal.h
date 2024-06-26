#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XFixedDecimal.d.h"






ICU4XFixedDecimal* ICU4XFixedDecimal_new(int32_t v);

void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

typedef struct ICU4XFixedDecimal_to_string_result { bool is_ok;} ICU4XFixedDecimal_to_string_result;
ICU4XFixedDecimal_to_string_result ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWrite* write);


void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);





#endif // ICU4XFixedDecimal_H
