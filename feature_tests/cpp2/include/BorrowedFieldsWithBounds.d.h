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
  DiplomatString16View field_a;
  DiplomatStringView field_b;
  DiplomatStringView field_c;
} BorrowedFieldsWithBounds;



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // BorrowedFieldsWithBounds_D_H
