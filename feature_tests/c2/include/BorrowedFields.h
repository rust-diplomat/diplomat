#ifndef BorrowedFields_H
#define BorrowedFields_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"
#include "Bar.h"

#include "BorrowedFields.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


BorrowedFields BorrowedFields_from_bar_and_strings(const Bar* bar, const char16_t* dstr16_data, size_t dstr16_len, const char* utf8_str_data, size_t utf8_str_len);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // BorrowedFields_H
