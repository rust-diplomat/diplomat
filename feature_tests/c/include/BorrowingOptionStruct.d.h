#ifndef BorrowingOptionStruct_D_H
#define BorrowingOptionStruct_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BorrowingOptionStruct {
  OptionStringView a;
} BorrowingOptionStruct;

typedef struct BorrowingOptionStruct_option {union { BorrowingOptionStruct ok; }; bool is_ok; } BorrowingOptionStruct_option;



#endif // BorrowingOptionStruct_D_H
