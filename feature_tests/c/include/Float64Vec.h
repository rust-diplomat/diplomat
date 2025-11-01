#ifndef Float64Vec_H
#define Float64Vec_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Float64Vec.d.h"






Float64Vec* Float64Vec_new(DiplomatF64View v);

Float64Vec* Float64Vec_new_bool(DiplomatBoolView v);

Float64Vec* Float64Vec_new_i16(DiplomatI16View v);

Float64Vec* Float64Vec_new_u16(DiplomatU16View v);

Float64Vec* Float64Vec_new_isize(DiplomatIsizeView v);

Float64Vec* Float64Vec_new_usize(DiplomatUsizeView v);

Float64Vec* Float64Vec_new_f64_be_bytes(DiplomatU8View v);

DiplomatF64View Float64Vec_as_slice(const Float64Vec* self);

void Float64Vec_fill_slice(const Float64Vec* self, DiplomatF64ViewMut v);

void Float64Vec_set_value(Float64Vec* self, DiplomatF64View new_slice);

void Float64Vec_to_string(const Float64Vec* self, DiplomatWrite* write);

DiplomatF64View Float64Vec_borrow(const Float64Vec* self);

typedef struct Float64Vec_get_result {union {double ok; }; bool is_ok;} Float64Vec_get_result;
Float64Vec_get_result Float64Vec_get(const Float64Vec* self, size_t i);

void Float64Vec_destroy(Float64Vec* self);





#endif // Float64Vec_H
