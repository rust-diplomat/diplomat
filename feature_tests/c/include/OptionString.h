#ifndef OptionString_H
#define OptionString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OptionString.d.h"






OptionString* OptionString_new(DiplomatStringView diplomat_str);

typedef struct OptionString_write_result { bool is_ok;} OptionString_write_result;
OptionString_write_result OptionString_write(const OptionString* self, DiplomatWrite* write);

typedef struct OptionString_borrow_result {union {DiplomatStringView ok; }; bool is_ok;} OptionString_borrow_result;
OptionString_borrow_result OptionString_borrow(const OptionString* self);

void OptionString_destroy(OptionString* self);





#endif // OptionString_H
