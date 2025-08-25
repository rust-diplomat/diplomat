#ifndef NestedBorrowedFields_D_H
#define NestedBorrowedFields_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BorrowedFields.d.h"
#include "BorrowedFieldsWithBounds.d.h"



typedef struct NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;
} NestedBorrowedFields;

typedef struct NestedBorrowedFields_option {union { NestedBorrowedFields ok; }; bool is_ok; } NestedBorrowedFields_option;

#endif // NestedBorrowedFields_D_H
