#ifndef BorrowedFieldsReturning_HPP
#define BorrowedFieldsReturning_HPP

#include "BorrowedFieldsReturning.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::BorrowedFieldsReturning BorrowedFieldsReturning::AsFFI() const {
  return capi::BorrowedFieldsReturning {
    .bytes = { .data = bytes.data(), .len = bytes.size() },
  };
}

inline BorrowedFieldsReturning BorrowedFieldsReturning::FromFFI(capi::BorrowedFieldsReturning c_struct) {
  return BorrowedFieldsReturning {
    .bytes = std::string_view(c_struct.bytes.data, c_struct.bytes.len),
  };
}


#endif // BorrowedFieldsReturning_HPP
