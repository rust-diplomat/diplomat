#ifndef MyString_H
#define MyString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyString.d.h"

namespace capi {


extern "C" {

MyString* MyString_new(const char* v_data, size_t v_len);

MyString* MyString_new_unsafe(const char* v_data, size_t v_len);

MyString* MyString_new_owned(const char* v_data, size_t v_len);

MyString* MyString_new_from_first(DiplomatStringsView* v_data, size_t v_len);

void MyString_set_str(MyString* self, const char* new_str_data, size_t new_str_len);

void MyString_get_str(const MyString* self, DiplomatWrite* write);

DiplomatStringView MyString_get_boxed_str(const MyString* self);


void MyString_destroy(MyString* self);

} // extern "C"

} // namespace capi

#endif // MyString_H
