#ifndef BorrowedFields_H
#define BorrowedFields_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct BorrowedFields {
    DiplomatU16View a;
    DiplomatStringView b;
    DiplomatStringView c;
} BorrowedFields;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Bar.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

BorrowedFields BorrowedFields_from_bar_and_strings(const Bar* bar, const char16_t* dstr16_data, size_t dstr16_len, const char* utf8_str_data, size_t utf8_str_len);
void BorrowedFields_destroy(BorrowedFields* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
