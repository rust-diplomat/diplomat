#ifndef OptionString_H
#define OptionString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OptionString.d.h"






OptionString* OptionString_new(const char* diplomat_str_data, size_t diplomat_str_len);

struct OptionString_write_result { bool is_ok;};
struct OptionString_write_result OptionString_write(const OptionString* self);

struct OptionString_borrow_result {union {DiplomatStringView ok; }; bool is_ok;};
struct OptionString_borrow_result OptionString_borrow(const OptionString* self);


void OptionString_destroy(OptionString* self);





#endif // OptionString_H
