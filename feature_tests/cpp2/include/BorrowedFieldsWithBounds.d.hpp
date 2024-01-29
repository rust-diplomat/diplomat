#ifndef BorrowedFieldsWithBounds_D_HPP
#define BorrowedFieldsWithBounds_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFieldsWithBounds.d.h"


struct BorrowedFieldsWithBounds {
  std::u16string_view field_a;
  std::string_view field_b;
  std::string_view field_c;

  inline capi::BorrowedFieldsWithBounds AsFFI() const;
  inline static BorrowedFieldsWithBounds FromFFI(capi::BorrowedFieldsWithBounds c_struct);
};


#endif // BorrowedFieldsWithBounds_D_HPP
