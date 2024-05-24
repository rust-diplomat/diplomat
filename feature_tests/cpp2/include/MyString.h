#ifndef MyString_H
#define MyString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_box_MyString_Utf8Error.d.h"

#include "MyString.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


MyString* MyString_new(const char* v_data, size_t v_len);

diplomat_result_box_MyString_Utf8Error MyString_new_2(const char* v_data, size_t v_len);

MyString* MyString_new_unsafe(const char* v_data, size_t v_len);

MyString* MyString_new_owned(const char* v_data, size_t v_len);

MyString* MyString_new_from_first(DiplomatStrs8View* v_data, size_t v_len);

void MyString_set_str(MyString* self, const char* new_str_data, size_t new_str_len);

void MyString_get_str(const MyString* self, DiplomatWriteable* writeable);

struct { const char* data; size_t len; } MyString_get_boxed_str(const MyString* self);

void MyString_destroy(MyString* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // MyString_H
