#ifndef BorrowedFieldsReturning_D_H
#define BorrowedFieldsReturning_D_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct BorrowedFieldsReturning {
  struct { const uint8_t* data; size_t len; } bytes;
} BorrowedFieldsReturning;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // BorrowedFieldsReturning_D_H
