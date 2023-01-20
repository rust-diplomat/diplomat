#ifndef BorrowedFieldsReturning_HPP
#define BorrowedFieldsReturning_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFieldsReturning.h"

#include "BorrowedFieldsReturning.d.hpp"



inline capi::BorrowedFieldsReturning BorrowedFieldsReturning::AsFFI() const {
  return capi::BorrowedFieldsReturning {
    .bytes_data = bytes.data(),
    .bytes_size = bytes.size(),
  };
}

inline BorrowedFieldsReturning BorrowedFieldsReturning::FromFFI(capi::BorrowedFieldsReturning c_struct) {
  return BorrowedFieldsReturning {
    .bytes = std::span<const uint8_t>(c_struct.bytes_data, c_struct.bytes_size),
  };
}


#endif // BorrowedFieldsReturning_HPP
