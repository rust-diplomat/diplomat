#ifndef NestedBorrowedFields_D_HPP
#define NestedBorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsWithBounds.d.hpp"
#include "NestedBorrowedFields.d.h"

struct BorrowedFields;
struct BorrowedFieldsWithBounds;


struct NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  inline capi::NestedBorrowedFields AsFFI() const;
  inline static NestedBorrowedFields FromFFI(capi::NestedBorrowedFields c_struct);
};


#endif // NestedBorrowedFields_D_HPP
