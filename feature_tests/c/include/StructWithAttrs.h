#ifndef StructWithAttrs_H
#define StructWithAttrs_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "StructWithAttrs.d.h"






typedef struct namespace_StructWithAttrs_new_fallible_result {union {StructWithAttrs ok; }; bool is_ok;} namespace_StructWithAttrs_new_fallible_result;
namespace_StructWithAttrs_new_fallible_result namespace_StructWithAttrs_new_fallible(bool a, uint32_t b);

uint32_t namespace_StructWithAttrs_c(StructWithAttrs self);

void namespace_StructWithAttrs_deprecated(StructWithAttrs self);





#endif // StructWithAttrs_H
