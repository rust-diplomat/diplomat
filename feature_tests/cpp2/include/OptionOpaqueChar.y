#ifndef OptionOpaqueChar_H
#define OptionOpaqueChar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct OptionOpaqueChar OptionOpaqueChar;



void OptionOpaqueChar_assert_char(const OptionOpaqueChar* self, char32_t ch);
void OptionOpaqueChar_destroy(OptionOpaqueChar* self);


#endif // OptionOpaqueChar_HPP
