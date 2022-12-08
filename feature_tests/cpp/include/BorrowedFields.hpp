#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "BorrowedFields.h"


struct BorrowedFields {
 public:
  std::string_view b;
};


#endif
