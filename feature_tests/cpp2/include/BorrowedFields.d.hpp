#ifndef BorrowedFields_D_HPP
#define BorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class Bar;


namespace capi {
    typedef struct BorrowedFields {
      DiplomatString16View a;
      DiplomatStringView b;
      DiplomatStringView c;
    } BorrowedFields;
}

struct BorrowedFields {
  std::u16string_view a;
  std::string_view b;
  std::string_view c;

  inline static diplomat::result<BorrowedFields, diplomat::Utf8Error> from_bar_and_strings(const Bar& bar, std::u16string_view dstr16, std::string_view utf8_str);

  inline capi::BorrowedFields AsFFI() const;
  inline static BorrowedFields FromFFI(capi::BorrowedFields c_struct);
};


#endif // BorrowedFields_D_HPP
