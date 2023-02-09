#ifndef BorrowedFieldsReturning_D_HPP
#define BorrowedFieldsReturning_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFieldsReturning.d.h"


struct BorrowedFieldsReturning {
  diplomat::span<const uint8_t> bytes;

  inline capi::BorrowedFieldsReturning AsFFI() const;
  inline static BorrowedFieldsReturning FromFFI(capi::BorrowedFieldsReturning c_struct);
};


#endif // BorrowedFieldsReturning_D_HPP
