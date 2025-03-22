#ifndef BorrowedFieldsWithBounds_HPP
#define BorrowedFieldsWithBounds_HPP

#include "BorrowedFieldsWithBounds.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const diplomat::capi::Foo* foo, diplomat::capi::DiplomatString16View dstr16_x, diplomat::capi::DiplomatStringView utf8_str_z);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<BorrowedFieldsWithBounds, diplomat::Utf8Error> BorrowedFieldsWithBounds::from_foo_and_strings(const Foo& foo, std::u16string_view dstr16_x, std::string_view utf8_str_z) {
  if (!diplomat::capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::BorrowedFieldsWithBounds_from_foo_and_strings(foo.AsFFI(),
    {dstr16_x.data(), dstr16_x.size()},
    {utf8_str_z.data(), utf8_str_z.size()});
  return diplomat::Ok<BorrowedFieldsWithBounds>(BorrowedFieldsWithBounds::FromFFI(result));
}


inline diplomat::capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds::AsFFI() const {
  return diplomat::capi::BorrowedFieldsWithBounds {
    /* .field_a = */ {field_a.data(), field_a.size()},
    /* .field_b = */ {field_b.data(), field_b.size()},
    /* .field_c = */ {field_c.data(), field_c.size()},
  };
}

inline BorrowedFieldsWithBounds BorrowedFieldsWithBounds::FromFFI(diplomat::capi::BorrowedFieldsWithBounds c_struct) {
  return BorrowedFieldsWithBounds {
    /* .field_a = */ std::u16string_view(c_struct.field_a.data, c_struct.field_a.len),
    /* .field_b = */ std::string_view(c_struct.field_b.data, c_struct.field_b.len),
    /* .field_c = */ std::string_view(c_struct.field_c.data, c_struct.field_c.len),
  };
}


#endif // BorrowedFieldsWithBounds_HPP
