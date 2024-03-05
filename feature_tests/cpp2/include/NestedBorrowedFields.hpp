#ifndef NestedBorrowedFields_HPP
#define NestedBorrowedFields_HPP

#include "NestedBorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "NestedBorrowedFields.h"



inline capi::NestedBorrowedFields NestedBorrowedFields::AsFFI() const {
  return capi::NestedBorrowedFields {
    .fields = fields.AsFFI(),
    .bounds = bounds.AsFFI(),
    .bounds2 = bounds2.AsFFI(),
  };
}

inline NestedBorrowedFields NestedBorrowedFields::FromFFI(capi::NestedBorrowedFields c_struct) {
  return NestedBorrowedFields {
    .fields = BorrowedFields::FromFFI(c_struct.fields),
    .bounds = BorrowedFieldsWithBounds::FromFFI(c_struct.bounds),
    .bounds2 = BorrowedFieldsWithBounds::FromFFI(c_struct.bounds2),
  };
}


#endif // NestedBorrowedFields_HPP
