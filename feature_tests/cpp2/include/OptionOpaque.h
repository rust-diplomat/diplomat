#ifndef OptionOpaque_H
#define OptionOpaque_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionStruct.d.h"
#include "OptionStruct.h"

#include "OptionOpaque.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


OptionOpaque* OptionOpaque_new(int32_t i);

OptionOpaque* OptionOpaque_new_none();

OptionStruct OptionOpaque_new_struct();

OptionStruct OptionOpaque_new_struct_nones();

void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);

bool OptionOpaque_option_opaque_argument(const OptionOpaque* arg);

void OptionOpaque_destroy(OptionOpaque* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // OptionOpaque_H
