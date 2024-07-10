#ifndef BorrowedFieldsWithBounds_HPP
#define BorrowedFieldsWithBounds_HPP

#include "BorrowedFieldsWithBounds.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Foo.hpp"


namespace capi {
    extern "C" {
    
    ::capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const ::capi::Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char* utf8_str_z_data, size_t utf8_str_z_len);
    
    
    } // extern "C"
}
inline diplomat::result<BorrowedFieldsWithBounds, diplomat::Utf8Error> BorrowedFieldsWithBounds::from_foo_and_strings(const Foo& foo, std::u16string_view dstr16_x, std::string_view utf8_str_z) {
  if (!diplomat::capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::BorrowedFieldsWithBounds_from_foo_and_strings(foo.AsFFI(),
    dstr16_x.data(),
    dstr16_x.size(),
    utf8_str_z.data(),
    utf8_str_z.size());
  return diplomat::Ok<BorrowedFieldsWithBounds>(std::move(BorrowedFieldsWithBounds::FromFFI(result)));
}


inline ::capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds::AsFFI() const {
  return ::capi::BorrowedFieldsWithBounds {
    .field_a = { .data = field_a.data(), .len = field_a.size() },
    .field_b = { .data = field_b.data(), .len = field_b.size() },
    .field_c = { .data = field_c.data(), .len = field_c.size() },
  };
}

inline BorrowedFieldsWithBounds BorrowedFieldsWithBounds::FromFFI(::capi::BorrowedFieldsWithBounds c_struct) {
  return BorrowedFieldsWithBounds {
    .field_a = std::u16string_view(c_struct.field_a.data, c_struct.field_a.len),
    .field_b = std::string_view(c_struct.field_b.data, c_struct.field_b.len),
    .field_c = std::string_view(c_struct.field_c.data, c_struct.field_c.len),
  };
}


#endif // BorrowedFieldsWithBounds_HPP
