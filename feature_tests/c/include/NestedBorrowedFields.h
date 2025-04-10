#ifndef NestedBorrowedFields_H
#define NestedBorrowedFields_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"
#include "Foo.d.h"

#include "NestedBorrowedFields.d.h"






NestedBorrowedFields NestedBorrowedFields_from_bar_and_foo_and_strings(const Bar* bar, const Foo* foo, DiplomatString16View dstr16_x, DiplomatString16View dstr16_z, DiplomatStringView utf8_str_y, DiplomatStringView utf8_str_z);





#endif // NestedBorrowedFields_H
