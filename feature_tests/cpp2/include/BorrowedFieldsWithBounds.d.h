#ifndef BorrowedFieldsWithBounds_D_H
#define BorrowedFieldsWithBounds_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct BorrowedFieldsWithBounds {
  struct { const char16_t* data; size_t len; } field_a;
  struct { const char* data; size_t len; } field_b;
  struct { const char* data; size_t len; } field_c;
} BorrowedFieldsWithBounds;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // BorrowedFieldsWithBounds_D_H
