#ifndef Utf16Wrap_H
#define Utf16Wrap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Utf16Wrap.d.h"

namespace capi {


extern "C" {

DiplomatString16View Utf16Wrap_borrow_cont(const Utf16Wrap* self);

DiplomatString16View Utf16Wrap_owned(const Utf16Wrap* self);


void Utf16Wrap_destroy(Utf16Wrap* self);

} // extern "C"

} // namespace capi

#endif // Utf16Wrap_H
