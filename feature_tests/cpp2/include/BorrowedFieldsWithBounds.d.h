#ifndef BorrowedFieldsWithBounds_D_H
#define BorrowedFieldsWithBounds_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


namespace capi {


typedef struct BorrowedFieldsWithBounds {
  DiplomatString16View field_a;
  DiplomatStringView field_b;
  DiplomatStringView field_c;
} BorrowedFieldsWithBounds;



} // namespace capi

#endif // BorrowedFieldsWithBounds_D_H
