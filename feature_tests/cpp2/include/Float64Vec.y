#ifndef Float64Vec_H
#define Float64Vec_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct Float64Vec Float64Vec;



Float64Vec* Float64Vec_new(const double* v_data, size_t v_len);
void Float64Vec_fill_slice(const Float64Vec* self, double* v_data, size_t v_len);
void Float64Vec_set_value(Float64Vec* self, const double* new_slice_data, size_t new_slice_len);
void Float64Vec_destroy(Float64Vec* self);


#endif // Float64Vec_HPP
