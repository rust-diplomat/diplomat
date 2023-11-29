#ifndef BorrowedFields_D_HPP
#define BorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.d.h"


struct BorrowedFields {
  std::u16string_view a;
  std::string_view b;
  std::string_view c;

  inline capi::BorrowedFields AsFFI() const;
  inline static BorrowedFields FromFFI(capi::BorrowedFields c_struct);
};


#endif // BorrowedFields_D_HPP
