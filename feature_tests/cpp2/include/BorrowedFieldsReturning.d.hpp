#ifndef BorrowedFieldsReturning_D_HPP
#define BorrowedFieldsReturning_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct BorrowedFieldsReturning {
      DiplomatStringView bytes;
    } BorrowedFieldsReturning;
}struct BorrowedFieldsReturning {
  std::string_view bytes;

  inline capi::BorrowedFieldsReturning AsFFI() const;
  inline static BorrowedFieldsReturning FromFFI(capi::BorrowedFieldsReturning c_struct);
};


#endif // BorrowedFieldsReturning_D_HPP
