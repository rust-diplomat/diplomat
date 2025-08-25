#ifndef BorrowedFieldsWithBounds_D_H
#define BorrowedFieldsWithBounds_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct BorrowedFieldsWithBounds {
  DiplomatString16View field_a;
  DiplomatStringView field_b;
  DiplomatStringView field_c;
} BorrowedFieldsWithBounds;

typedef struct BorrowedFieldsWithBounds_option {union { BorrowedFieldsWithBounds ok; }; bool is_ok; } BorrowedFieldsWithBounds_option;

#endif // BorrowedFieldsWithBounds_D_H
