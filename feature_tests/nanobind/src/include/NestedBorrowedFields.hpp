#ifndef NestedBorrowedFields_HPP
#define NestedBorrowedFields_HPP

#include "NestedBorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::NestedBorrowedFields NestedBorrowedFields_from_bar_and_foo_and_strings(const diplomat::capi::Bar* bar, const diplomat::capi::Foo* foo, diplomat::capi::DiplomatString16View dstr16_x, diplomat::capi::DiplomatString16View dstr16_z, diplomat::capi::DiplomatStringView utf8_str_y, diplomat::capi::DiplomatStringView utf8_str_z);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<NestedBorrowedFields, diplomat::Utf8Error> NestedBorrowedFields::from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z) {
  if (!diplomat::capi::diplomat_is_str(utf8_str_y.data(), utf8_str_y.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  if (!diplomat::capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::NestedBorrowedFields_from_bar_and_foo_and_strings(bar.AsFFI(),
    foo.AsFFI(),
    {dstr16_x.data(), dstr16_x.size()},
    {dstr16_z.data(), dstr16_z.size()},
    {utf8_str_y.data(), utf8_str_y.size()},
    {utf8_str_z.data(), utf8_str_z.size()});
  return diplomat::Ok<NestedBorrowedFields>(NestedBorrowedFields::FromFFI(result));
}


inline diplomat::capi::NestedBorrowedFields NestedBorrowedFields::AsFFI() const {
  return diplomat::capi::NestedBorrowedFields {
    /* .fields = */ fields.AsFFI(),
    /* .bounds = */ bounds.AsFFI(),
    /* .bounds2 = */ bounds2.AsFFI(),
  };
}

inline NestedBorrowedFields NestedBorrowedFields::FromFFI(diplomat::capi::NestedBorrowedFields c_struct) {
  return NestedBorrowedFields {
    /* .fields = */ BorrowedFields::FromFFI(c_struct.fields),
    /* .bounds = */ BorrowedFieldsWithBounds::FromFFI(c_struct.bounds),
    /* .bounds2 = */ BorrowedFieldsWithBounds::FromFFI(c_struct.bounds2),
  };
}


#endif // NestedBorrowedFields_HPP
