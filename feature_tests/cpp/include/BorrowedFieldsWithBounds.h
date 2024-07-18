#ifndef BorrowedFieldsWithBounds_H
#define BorrowedFieldsWithBounds_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct BorrowedFieldsWithBounds {
    DiplomatU16View field_a;
    DiplomatStringView field_b;
    DiplomatStringView field_c;
} BorrowedFieldsWithBounds;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Foo.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char* utf8_str_z_data, size_t utf8_str_z_len);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
