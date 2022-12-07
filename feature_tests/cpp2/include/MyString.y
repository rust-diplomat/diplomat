#ifndef MyString_H
#define MyString_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct MyString MyString;



std::unique_ptr<MyString> MyString_new(std::string_view v);
void MyString_set_str(MyString& self, std::string_view new_str);
void MyString_get_str(const MyString& self, DiplomatWriteable* writeable);
void MyString_destroy(MyString* self);


#endif // MyString_HPP
