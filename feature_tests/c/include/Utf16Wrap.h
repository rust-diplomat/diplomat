#ifndef Utf16Wrap_H
#define Utf16Wrap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Utf16Wrap.d.h"






Utf16Wrap* Utf16Wrap_from_utf16(DiplomatString16View input);

void Utf16Wrap_get_debug_str(const Utf16Wrap* self, DiplomatWrite* write);

DiplomatString16View Utf16Wrap_borrow_cont(const Utf16Wrap* self);

void Utf16Wrap_destroy(Utf16Wrap* self);





#endif // Utf16Wrap_H
