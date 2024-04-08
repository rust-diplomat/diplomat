#ifndef Float64Vec_H
#define Float64Vec_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct Float64Vec Float64Vec;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_double_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

Float64Vec* Float64Vec_new(const double* v_data, size_t v_len);

Float64Vec* Float64Vec_new_bool(const bool* v_data, size_t v_len);

Float64Vec* Float64Vec_new_i16(const int16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_u16(const uint16_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_isize(const intptr_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_usize(const size_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_f64_be_bytes(const uint8_t* v_data, size_t v_len);

Float64Vec* Float64Vec_new_from_owned(double* v_data, size_t v_len);

DiplomatF64Array Float64Vec_as_boxed_slice(const Float64Vec* self);

DiplomatF64View Float64Vec_as_slice(const Float64Vec* self);

void Float64Vec_fill_slice(const Float64Vec* self, double* v_data, size_t v_len);

void Float64Vec_set_value(Float64Vec* self, const double* new_slice_data, size_t new_slice_len);

void Float64Vec_to_string(const Float64Vec* self, DiplomatWriteable* w);

DiplomatF64View Float64Vec_borrow(const Float64Vec* self);

diplomat_result_double_void Float64Vec_get(const Float64Vec* self, size_t i);
void Float64Vec_destroy(Float64Vec* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
