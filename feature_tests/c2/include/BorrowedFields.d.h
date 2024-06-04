#ifndef BorrowedFields_D_H
#define BorrowedFields_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BorrowedFields {
  DiplomatString16View a;
  DiplomatStringView b;
  DiplomatStringView c;
} BorrowedFields;





#endif // BorrowedFields_D_H
