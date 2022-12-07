#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_void_void.hpp"




class ICU4XFixedDecimal;



std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal_new(int32_t v);
void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal& self, int16_t power);
void ICU4XFixedDecimal_negate(ICU4XFixedDecimal& self);
diplomat_result_void_void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal& self, DiplomatWriteable* writeable);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);


#endif // ICU4XFixedDecimal_HPP
