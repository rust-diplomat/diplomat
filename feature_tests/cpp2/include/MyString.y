#ifndef MyString_H
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct MyString MyString;



MyString* MyString_new(const char* v_data, size_t v_len);
void MyString_set_str(MyString* self, const char* new_str_data, size_t new_str_len);
void MyString_get_str(const MyString* self, DiplomatWriteable* writeable);
void MyString_destroy(MyString* self);


#endif // MyString_HPP
