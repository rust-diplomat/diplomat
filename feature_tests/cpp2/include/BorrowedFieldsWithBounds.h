#ifndef BorrowedFieldsWithBounds_H
#define BorrowedFieldsWithBounds_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Foo.d.h"

#include "BorrowedFieldsWithBounds.d.h"

namespace capi {


extern "C" {

BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char* utf8_str_z_data, size_t utf8_str_z_len);


} // extern "C"

} // namespace capi

#endif // BorrowedFieldsWithBounds_H
