#ifndef BorrowedFieldsReturning_D_H
#define BorrowedFieldsReturning_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BorrowedFieldsReturning {
  DiplomatStringView bytes;
} BorrowedFieldsReturning;

typedef struct BorrowedFieldsReturning_option {union { BorrowedFieldsReturning ok; }; bool is_ok; } BorrowedFieldsReturning_option;





#endif // BorrowedFieldsReturning_D_H
