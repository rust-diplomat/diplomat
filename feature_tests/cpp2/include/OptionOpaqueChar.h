#ifndef OptionOpaqueChar_H
#define OptionOpaqueChar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OptionOpaqueChar.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



void OptionOpaqueChar_assert_char(const OptionOpaqueChar* self, char32_t ch);

void OptionOpaqueChar_destroy(OptionOpaqueChar* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // OptionOpaqueChar_H
