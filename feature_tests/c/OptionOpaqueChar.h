#ifndef OptionOpaqueChar_H
#define OptionOpaqueChar_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OptionOpaqueChar OptionOpaqueChar;

void OptionOpaqueChar_assert_char(const OptionOpaqueChar* self, char32_t ch);
void OptionOpaqueChar_destroy(OptionOpaqueChar* self);

#ifdef __cplusplus
}
#endif
#endif
