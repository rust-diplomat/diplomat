#ifndef BorrowedFieldsReturning_D_HPP
#define BorrowedFieldsReturning_D_HPP

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
    struct BorrowedFieldsReturning {
      diplomat::capi::DiplomatStringView bytes;
    };
    
    typedef struct BorrowedFieldsReturning_option {union { BorrowedFieldsReturning ok; }; bool is_ok; } BorrowedFieldsReturning_option;
} // namespace capi
} // namespace


struct BorrowedFieldsReturning {
  std::string_view bytes;

  inline diplomat::capi::BorrowedFieldsReturning AsFFI() const;
  inline static BorrowedFieldsReturning FromFFI(diplomat::capi::BorrowedFieldsReturning c_struct);
};


#endif // BorrowedFieldsReturning_D_HPP
