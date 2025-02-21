#ifndef MyString_H
#define MyString_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MyString.d.h"






MyString* MyString_new(DiplomatStringView v);

MyString* MyString_new_unsafe(DiplomatStringView v);

MyString* MyString_new_owned(DiplomatStringView v);

MyString* MyString_new_from_first(DiplomatStringsView v);

void MyString_set_str(MyString* self, DiplomatStringView new_str);

void MyString_get_str(const MyString* self, DiplomatWrite* write);

DiplomatStringView MyString_get_static_str(void);

void MyString_string_transform(DiplomatStringView foo, DiplomatWrite* write);

DiplomatStringView MyString_borrow(const MyString* self);


void MyString_destroy(MyString* self);





#endif // MyString_H
