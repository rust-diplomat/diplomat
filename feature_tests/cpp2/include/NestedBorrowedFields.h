#ifndef NestedBorrowedFields_H
#define NestedBorrowedFields_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Bar.d.h"
#include "Bar.h"
#include "Foo.d.h"
#include "Foo.h"

#include "NestedBorrowedFields.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


NestedBorrowedFields NestedBorrowedFields_from_bar_and_foo_and_strings(const Bar* bar, const Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char16_t* dstr16_z_data, size_t dstr16_z_len, const char* utf8_str_y_data, size_t utf8_str_y_len, const char* utf8_str_z_data, size_t utf8_str_z_len);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // NestedBorrowedFields_H
