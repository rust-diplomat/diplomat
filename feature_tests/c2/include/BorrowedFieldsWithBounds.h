#ifndef BorrowedFieldsWithBounds_H
#define BorrowedFieldsWithBounds_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Foo.d.h"
#include "Foo.h"

#include "BorrowedFieldsWithBounds.d.h"






BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char* utf8_str_z_data, size_t utf8_str_z_len);






#endif // BorrowedFieldsWithBounds_H
