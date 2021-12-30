#ifndef OptionOpaque_H
#define OptionOpaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OptionOpaque OptionOpaque;
#include "OptionStruct.h"

OptionOpaque* OptionOpaque_new(int32_t i);

OptionOpaque* OptionOpaque_new_none();

OptionStruct OptionOpaque_new_struct();

OptionStruct OptionOpaque_new_struct_nones();

void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);
void OptionOpaque_destroy(OptionOpaque* self);

#ifdef __cplusplus
}
#endif
#endif
