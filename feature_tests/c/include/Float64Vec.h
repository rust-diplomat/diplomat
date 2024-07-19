#ifndef Float64Vec_H
#define Float64Vec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Float64Vec.d.h"






Float64Vec* Float64Vec_new(const double* v_data, size_t v_len);

Float64Vec* Float64Vec_new_bool(const bool* v_data, size_t v_len);

Float64Vec* Float64Vec_new_i16(const int16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_u16(const uint16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_isize(const intptr_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_usize(const size_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_f64_be_bytes(const uint8_t* v_data, size_t v_len);

DiplomatF64View Float64Vec_as_slice(const Float64Vec* self);

void Float64Vec_fill_slice(const Float64Vec* self, double* v_data, size_t v_len);

void Float64Vec_set_value(Float64Vec* self, const double* new_slice_data, size_t new_slice_len);

void Float64Vec_to_string(const Float64Vec* self, DiplomatWrite* write);

DiplomatF64View Float64Vec_borrow(const Float64Vec* self);

typedef struct Float64Vec_get_result {union {double ok; }; bool is_ok;} Float64Vec_get_result;
Float64Vec_get_result Float64Vec_get(const Float64Vec* self, size_t i);


void Float64Vec_destroy(Float64Vec* self);





#endif // Float64Vec_H
