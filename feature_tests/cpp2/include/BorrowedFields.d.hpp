#ifndef BorrowedFields_D_HPP
#define BorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct Bar Bar; }
class Bar;


namespace diplomat {
namespace capi {
    typedef struct BorrowedFields {
      DiplomatString16View a;
      DiplomatStringView b;
      DiplomatStringView c;
    } BorrowedFields;
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
