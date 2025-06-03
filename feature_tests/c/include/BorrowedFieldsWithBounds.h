#ifndef BorrowedFieldsWithBounds_H
#define BorrowedFieldsWithBounds_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Foo.d.h"

#include "BorrowedFieldsWithBounds.d.h"






BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const Foo* foo, DiplomatString16View dstr16_x, DiplomatStringView utf8_str_z);





#endif // BorrowedFieldsWithBounds_H
