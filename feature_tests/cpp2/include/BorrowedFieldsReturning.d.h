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
  DiplomatStringView bytes;
} BorrowedFieldsReturning;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // BorrowedFieldsReturning_D_H
