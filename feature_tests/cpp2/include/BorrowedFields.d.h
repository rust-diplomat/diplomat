#ifndef BorrowedFields_D_H
#define BorrowedFields_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


namespace capi {


typedef struct BorrowedFields {
  DiplomatString16View a;
  DiplomatStringView b;
  DiplomatStringView c;
} BorrowedFields;



} // namespace capi

#endif // BorrowedFields_D_H
