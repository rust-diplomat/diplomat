#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP

#include "BorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Bar.hpp"
#include "BorrowedFields.h"


inline diplomat::result<BorrowedFields, diplomat::Utf8Error> BorrowedFields::from_bar_and_strings(const Bar& bar, std::u16string_view dstr16, std::string_view utf8_str) {
  if (!capi::is_str(utf8_str.data(), utf8_str.size()) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error)
  }
  auto result = capi::BorrowedFields_from_bar_and_strings(bar.AsFFI(),
    dstr16.data(),
    dstr16.size(),
    utf8_str.data(),
    utf8_str.size());
  return diplomat::Ok<BorrowedFields>(BorrowedFields::FromFFI(result));
}


inline capi::BorrowedFields BorrowedFields::AsFFI() const {
  return capi::BorrowedFields {
    .a_data = a.data(),
    .a_size = a.size(),
    .b_data = b.data(),
    .b_size = b.size(),
    .c_data = c.data(),
    .c_size = c.size(),
  };
}

inline BorrowedFields BorrowedFields::FromFFI(capi::BorrowedFields c_struct) {
  return BorrowedFields {
    .a = std::u16string_view(c_struct.a_data, c_struct.a_size),
    .b = std::string_view(c_struct.b_data, c_struct.b_size),
    .c = std::string_view(c_struct.c_data, c_struct.c_size),
  };
}


#endif // BorrowedFields_HPP
