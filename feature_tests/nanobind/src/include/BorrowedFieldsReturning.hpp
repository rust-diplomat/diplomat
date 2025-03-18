#ifndef BorrowedFieldsReturning_HPP
#define BorrowedFieldsReturning_HPP

#include "BorrowedFieldsReturning.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::BorrowedFieldsReturning BorrowedFieldsReturning::AsFFI() const {
  return diplomat::capi::BorrowedFieldsReturning {
    /* .bytes = */ {bytes.data(), bytes.size()},
  };
}

inline BorrowedFieldsReturning BorrowedFieldsReturning::FromFFI(diplomat::capi::BorrowedFieldsReturning c_struct) {
  return BorrowedFieldsReturning {
    /* .bytes = */ std::string_view(c_struct.bytes.data, c_struct.bytes.len),
  };
}


#endif // BorrowedFieldsReturning_HPP
