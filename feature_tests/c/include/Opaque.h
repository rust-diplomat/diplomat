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






Opaque* Opaque_new(void);

Opaque* Opaque_try_from_utf8(DiplomatStringView input);

Opaque* Opaque_from_str(DiplomatStringView input);

void Opaque_get_debug_str(const Opaque* self, DiplomatWrite* write);

void Opaque_assert_struct(const Opaque* self, MyStruct s);

size_t Opaque_returns_usize(void);

ImportedStruct Opaque_returns_imported(void);

int8_t Opaque_cmp(void);

void Opaque_destroy(Opaque* self);





#endif // Opaque_H
