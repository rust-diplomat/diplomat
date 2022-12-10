#ifndef BorrowedFields_D_H
#define BorrowedFields_D_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct BorrowedFields {
  struct { const uint16_t* data; size_t len; } a;
  struct { const char* data; size_t len; } b;
} BorrowedFields;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // BorrowedFields_D_H
