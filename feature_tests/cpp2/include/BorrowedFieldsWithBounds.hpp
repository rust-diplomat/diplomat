#ifndef BorrowedFieldsWithBounds_HPP
#define BorrowedFieldsWithBounds_HPP

#include "BorrowedFieldsWithBounds.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFieldsWithBounds.h"



inline capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds::AsFFI() const {
  return capi::BorrowedFieldsWithBounds {
    .field_a_data = field_a.data(),
    .field_a_size = field_a.size(),
    .field_b_data = field_b.data(),
    .field_b_size = field_b.size(),
    .field_c_data = field_c.data(),
    .field_c_size = field_c.size(),
  };
}

inline BorrowedFieldsWithBounds BorrowedFieldsWithBounds::FromFFI(capi::BorrowedFieldsWithBounds c_struct) {
  return BorrowedFieldsWithBounds {
    .field_a = std::u16string_view(c_struct.field_a_data, c_struct.field_a_size),
    .field_b = std::string_view(c_struct.field_b_data, c_struct.field_b_size),
    .field_c = std::string_view(c_struct.field_c_data, c_struct.field_c_size),
  };
}


#endif // BorrowedFieldsWithBounds_HPP
