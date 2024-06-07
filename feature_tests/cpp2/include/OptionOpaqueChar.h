#ifndef OptionOpaqueChar_H
#define OptionOpaqueChar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OptionOpaqueChar.d.h"

namespace capi {


extern "C" {

void OptionOpaqueChar_assert_char(const OptionOpaqueChar* self, char32_t ch);


void OptionOpaqueChar_destroy(OptionOpaqueChar* self);

} // extern "C"

} // namespace capi

#endif // OptionOpaqueChar_H
