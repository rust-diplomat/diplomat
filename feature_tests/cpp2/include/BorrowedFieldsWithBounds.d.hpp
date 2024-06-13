#ifndef BorrowedFieldsWithBounds_D_HPP
#define BorrowedFieldsWithBounds_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class Foo;


namespace capi {
    typedef struct BorrowedFieldsWithBounds {
      DiplomatString16View field_a;
      DiplomatStringView field_b;
      DiplomatStringView field_c;
    } BorrowedFieldsWithBounds;
}

struct BorrowedFieldsWithBounds {
  std::u16string_view field_a;
  std::string_view field_b;
  std::string_view field_c;

  inline static diplomat::result<BorrowedFieldsWithBounds, diplomat::Utf8Error> from_foo_and_strings(const Foo& foo, std::u16string_view dstr16_x, std::string_view utf8_str_z);

  inline capi::BorrowedFieldsWithBounds AsFFI() const;
  inline static BorrowedFieldsWithBounds FromFFI(capi::BorrowedFieldsWithBounds c_struct);
};


#endif // BorrowedFieldsWithBounds_D_HPP
