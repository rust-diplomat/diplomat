#ifndef BorrowedFields_D_HPP
#define BorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Bar; }
class Bar;


namespace diplomat {
namespace capi {
    struct BorrowedFields {
      diplomat::capi::DiplomatString16View a;
      diplomat::capi::DiplomatStringView b;
      diplomat::capi::DiplomatStringView c;
    };
    
    typedef struct BorrowedFields_option {union { BorrowedFields ok; }; bool is_ok; } BorrowedFields_option;
} // namespace capi
} // namespace


struct BorrowedFields {
  std::u16string_view a;
  std::string_view b;
  std::string_view c;

  inline static diplomat::result<BorrowedFields, diplomat::Utf8Error> from_bar_and_strings(const Bar& bar, std::u16string_view dstr16, std::string_view utf8_str);

  inline diplomat::capi::BorrowedFields AsFFI() const;
  inline static BorrowedFields FromFFI(diplomat::capi::BorrowedFields c_struct);
};


#endif // BorrowedFields_D_HPP
