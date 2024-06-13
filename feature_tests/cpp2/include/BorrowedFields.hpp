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


namespace capi {
    extern "C" {
    
    BorrowedFields BorrowedFields_from_bar_and_strings(const Bar* bar, const char16_t* dstr16_data, size_t dstr16_len, const char* utf8_str_data, size_t utf8_str_len);
    
    
    } // extern "C"
}

inline diplomat::result<BorrowedFields, diplomat::Utf8Error> BorrowedFields::from_bar_and_strings(const Bar& bar, std::u16string_view dstr16, std::string_view utf8_str) {
  if (!capi::diplomat_is_str(utf8_str.data(), utf8_str.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::BorrowedFields_from_bar_and_strings(bar.AsFFI(),
    dstr16.data(),
    dstr16.size(),
    utf8_str.data(),
    utf8_str.size());
  return diplomat::Ok<BorrowedFields>(std::move(BorrowedFields::FromFFI(result)));
}


inline capi::BorrowedFields BorrowedFields::AsFFI() const {
  return capi::BorrowedFields {
    .a = { .data = a.data(), .len = a.size() },
    .b = { .data = b.data(), .len = b.size() },
    .c = { .data = c.data(), .len = c.size() },
  };
}

inline BorrowedFields BorrowedFields::FromFFI(capi::BorrowedFields c_struct) {
  return BorrowedFields {
    .a = std::u16string_view(c_struct.a.data, c_struct.a.len),
    .b = std::string_view(c_struct.b.data, c_struct.b.len),
    .c = std::string_view(c_struct.c.data, c_struct.c.len),
  };
}


#endif // BorrowedFields_HPP
