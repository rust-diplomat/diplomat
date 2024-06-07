#ifndef BorrowedFieldsWithBounds_HPP
#define BorrowedFieldsWithBounds_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "BorrowedFieldsWithBounds.h"

class Foo;
struct BorrowedFieldsWithBounds;

struct BorrowedFieldsWithBounds {
 public:
  std::u16string_view field_a;
  std::string_view field_b;
  std::string_view field_c;

  /**
   * Lifetimes: `foo`, `dstr16_x`, `utf8_str_z` must live at least as long as the output.
   * 
   * 
   * Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).
   */
  static BorrowedFieldsWithBounds from_foo_and_strings(const Foo& foo, const std::u16string_view dstr16_x, const std::string_view utf8_str_z);
};

#include "Foo.hpp"

inline BorrowedFieldsWithBounds BorrowedFieldsWithBounds::from_foo_and_strings(const Foo& foo, const std::u16string_view dstr16_x, const std::string_view utf8_str_z) {
  capi::BorrowedFieldsWithBounds diplomat_raw_struct_out_value = capi::BorrowedFieldsWithBounds_from_foo_and_strings(foo.AsFFI(), dstr16_x.data(), dstr16_x.size(), utf8_str_z.data(), utf8_str_z.size());
  capi::DiplomatString16View diplomat_slice_raw_out_value_field_a = diplomat_raw_struct_out_value.field_a;
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value_field_a.data, diplomat_slice_raw_out_value_field_a.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_field_b = diplomat_raw_struct_out_value.field_b;
  std::string_view str(diplomat_str_raw_out_value_field_b.data, diplomat_str_raw_out_value_field_b.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_field_c = diplomat_raw_struct_out_value.field_c;
  std::string_view str(diplomat_str_raw_out_value_field_c.data, diplomat_str_raw_out_value_field_c.len);
  return BorrowedFieldsWithBounds{ .field_a = std::move(slice), .field_b = std::move(str), .field_c = std::move(str) };
}
#endif
