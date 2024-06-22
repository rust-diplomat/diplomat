#ifndef NestedBorrowedFields_HPP
#define NestedBorrowedFields_HPP

#include "NestedBorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"


namespace capi {
    extern "C" {
    
    NestedBorrowedFields NestedBorrowedFields_from_bar_and_foo_and_strings(const Bar* bar, const Foo* foo, const char16_t* dstr16_x_data, size_t dstr16_x_len, const char16_t* dstr16_z_data, size_t dstr16_z_len, const char* utf8_str_y_data, size_t utf8_str_y_len, const char* utf8_str_z_data, size_t utf8_str_z_len);
    
    
    } // extern "C"
}

inline diplomat::result<NestedBorrowedFields, diplomat::Utf8Error> NestedBorrowedFields::from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z) {
  if (!capi::diplomat_is_str(utf8_str_y.data(), utf8_str_y.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  if (!capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::NestedBorrowedFields_from_bar_and_foo_and_strings(bar.AsFFI(),
    foo.AsFFI(),
    dstr16_x.data(),
    dstr16_x.size(),
    dstr16_z.data(),
    dstr16_z.size(),
    utf8_str_y.data(),
    utf8_str_y.size(),
    utf8_str_z.data(),
    utf8_str_z.size());
  return diplomat::Ok<NestedBorrowedFields>(std::move(NestedBorrowedFields::FromFFI(result)));
}


inline capi::NestedBorrowedFields NestedBorrowedFields::AsFFI() const {
  return capi::NestedBorrowedFields {
    .fields = fields.AsFFI(),
    .bounds = bounds.AsFFI(),
    .bounds2 = bounds2.AsFFI(),
  };
}

inline NestedBorrowedFields NestedBorrowedFields::FromFFI(capi::NestedBorrowedFields c_struct) {
  return NestedBorrowedFields {
    .fields = BorrowedFields::FromFFI(c_struct.fields),
    .bounds = BorrowedFieldsWithBounds::FromFFI(c_struct.bounds),
    .bounds2 = BorrowedFieldsWithBounds::FromFFI(c_struct.bounds2),
  };
}


#endif // NestedBorrowedFields_HPP
