#ifndef Float64Vec_H
#define Float64Vec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Float64Vec.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


Float64Vec* Float64Vec_new(const double* v_data, size_t v_len);

Float64Vec* Float64Vec_new_bool(const bool* v_data, size_t v_len);

Float64Vec* Float64Vec_new_i16(const int16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_u16(const uint16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_isize(const intptr_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_usize(const size_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_f64_be_bytes(const uint8_t* v_data, size_t v_len);

void Float64Vec_fill_slice(const Float64Vec* self, double* v_data, size_t v_len);

void Float64Vec_set_value(Float64Vec* self, const double* new_slice_data, size_t new_slice_len);

void Float64Vec_to_string(const Float64Vec* self, DiplomatWriteable* writeable);

struct { const double* data; size_t len; } Float64Vec_borrow(const Float64Vec* self);

void Float64Vec_destroy(Float64Vec* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // Float64Vec_H
