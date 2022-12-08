#ifndef BorrowedFieldsReturning_HPP
#define BorrowedFieldsReturning_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "BorrowedFieldsReturning.h"


struct BorrowedFieldsReturning {
 public:
  const diplomat::span<const uint8_t> bytes;
};


#endif
