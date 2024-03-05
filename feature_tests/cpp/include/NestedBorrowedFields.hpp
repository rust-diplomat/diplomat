#ifndef NestedBorrowedFields_HPP
#define NestedBorrowedFields_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "NestedBorrowedFields.h"

#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"

struct NestedBorrowedFields {
 public:
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;
};


#endif
