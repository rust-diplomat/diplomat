#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP

#include "BorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Bar.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::BorrowedFields BorrowedFields_from_bar_and_strings(const diplomat::capi::Bar* bar, diplomat::capi::DiplomatString16View dstr16, diplomat::capi::DiplomatStringView utf8_str);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<BorrowedFields, diplomat::Utf8Error> BorrowedFields::from_bar_and_strings(const Bar& bar, std::u16string_view dstr16, std::string_view utf8_str) {
  if (!diplomat::capi::diplomat_is_str(utf8_str.data(), utf8_str.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::BorrowedFields_from_bar_and_strings(bar.AsFFI(),
    {dstr16.data(), dstr16.size()},
    {utf8_str.data(), utf8_str.size()});
  return diplomat::Ok<BorrowedFields>(BorrowedFields::FromFFI(result));
}


inline diplomat::capi::BorrowedFields BorrowedFields::AsFFI() const {
  return diplomat::capi::BorrowedFields {
    /* .a = */ {a.data(), a.size()},
    /* .b = */ {b.data(), b.size()},
    /* .c = */ {c.data(), c.size()},
  };
}

inline BorrowedFields BorrowedFields::FromFFI(diplomat::capi::BorrowedFields c_struct) {
  return BorrowedFields {
    /* .a = */ std::u16string_view(c_struct.a.data, c_struct.a.len),
    /* .b = */ std::string_view(c_struct.b.data, c_struct.b.len),
    /* .c = */ std::string_view(c_struct.c.data, c_struct.c.len),
  };
}


#endif // BorrowedFields_HPP
