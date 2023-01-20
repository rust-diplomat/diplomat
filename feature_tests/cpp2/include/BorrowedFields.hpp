#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.h"

#include "BorrowedFields.d.hpp"



inline capi::BorrowedFields BorrowedFields::AsFFI() const {
  return capi::BorrowedFields {
    .a_data = a.data(),
    .a_size = a.size(),
    .b_data = b.data(),
    .b_size = b.size(),
  };
}

inline BorrowedFields BorrowedFields::FromFFI(capi::BorrowedFields c_struct) {
  return BorrowedFields {
    .a = std::span<const uint16_t>(c_struct.a_data, c_struct.a_size),
    .b = std::string_view(c_struct.b_data, c_struct.b_size),
  };
}


#endif // BorrowedFields_HPP
