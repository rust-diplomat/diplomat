#ifndef Opaque_H
#define Opaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ImportedStruct.d.h"
#include "MyStruct.d.h"

#include "Opaque.d.h"

namespace capi {


extern "C" {

Opaque* Opaque_new();

Opaque* Opaque_try_from_utf8(const char* input_data, size_t input_len);

Opaque* Opaque_from_str(const char* input_data, size_t input_len);

void Opaque_get_debug_str(const Opaque* self, DiplomatWrite* write);

void Opaque_assert_struct(const Opaque* self, MyStruct s);

size_t Opaque_returns_usize();

ImportedStruct Opaque_returns_imported();

int8_t Opaque_cmp();


void Opaque_destroy(Opaque* self);

} // extern "C"

} // namespace capi

#endif // Opaque_H
