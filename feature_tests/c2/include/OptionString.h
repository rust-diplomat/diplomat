#ifndef OptionString_H
#define OptionString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "diplomat_result_str_ref8_void.d.h"
#include "diplomat_result_void_void.d.h"

#include "OptionString.d.h"






OptionString* OptionString_new(const char* diplomat_str_data, size_t diplomat_str_len);

diplomat_result_void_void OptionString_write(const OptionString* self, DiplomatWrite* write);

diplomat_result_str_ref8_void OptionString_borrow(const OptionString* self);


void OptionString_destroy(OptionString* self);





#endif // OptionString_H
