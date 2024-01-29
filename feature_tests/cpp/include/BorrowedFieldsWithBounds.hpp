#ifndef BorrowedFieldsWithBounds_HPP
#define BorrowedFieldsWithBounds_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "BorrowedFieldsWithBounds.h"


struct BorrowedFieldsWithBounds {
 public:
  std::u16string_view field_a;
  std::string_view field_b;
  std::string_view field_c;
};


#endif
