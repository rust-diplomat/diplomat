#ifndef NestedBorrowedFields_HPP
#define NestedBorrowedFields_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "NestedBorrowedFields.h"

#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
class Bar;
class Foo;
struct NestedBorrowedFields;

struct NestedBorrowedFields {
 public:
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  /**
   * Lifetimes: `bar`, `foo`, `dstr16_x`, `dstr16_z`, `utf8_str_y`, `utf8_str_z` must live at least as long as the output.
   * 
   * 
   * Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).
   */
  static NestedBorrowedFields from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, const std::u16string_view dstr16_x, const std::u16string_view dstr16_z, const std::string_view utf8_str_y, const std::string_view utf8_str_z);
};

#include "Bar.hpp"
#include "Foo.hpp"

inline NestedBorrowedFields NestedBorrowedFields::from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, const std::u16string_view dstr16_x, const std::u16string_view dstr16_z, const std::string_view utf8_str_y, const std::string_view utf8_str_z) {
  capi::NestedBorrowedFields diplomat_raw_struct_out_value = capi::NestedBorrowedFields_from_bar_and_foo_and_strings(bar.AsFFI(), foo.AsFFI(), dstr16_x.data(), dstr16_x.size(), dstr16_z.data(), dstr16_z.size(), utf8_str_y.data(), utf8_str_y.size(), utf8_str_z.data(), utf8_str_z.size());
  capi::BorrowedFields diplomat_raw_struct_out_value_fields = diplomat_raw_struct_out_value.fields;
  capi::DiplomatU16StringView diplomat_slice_raw_out_value_fields_a = diplomat_raw_struct_out_value_fields.a;
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value_fields_a.data, diplomat_slice_raw_out_value_fields_a.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_fields_b = diplomat_raw_struct_out_value_fields.b;
  std::string_view str(diplomat_str_raw_out_value_fields_b.data, diplomat_str_raw_out_value_fields_b.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_fields_c = diplomat_raw_struct_out_value_fields.c;
  std::string_view str(diplomat_str_raw_out_value_fields_c.data, diplomat_str_raw_out_value_fields_c.len);
  capi::BorrowedFieldsWithBounds diplomat_raw_struct_out_value_bounds = diplomat_raw_struct_out_value.bounds;
  capi::DiplomatU16StringView diplomat_slice_raw_out_value_bounds_field_a = diplomat_raw_struct_out_value_bounds.field_a;
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value_bounds_field_a.data, diplomat_slice_raw_out_value_bounds_field_a.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_bounds_field_b = diplomat_raw_struct_out_value_bounds.field_b;
  std::string_view str(diplomat_str_raw_out_value_bounds_field_b.data, diplomat_str_raw_out_value_bounds_field_b.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_bounds_field_c = diplomat_raw_struct_out_value_bounds.field_c;
  std::string_view str(diplomat_str_raw_out_value_bounds_field_c.data, diplomat_str_raw_out_value_bounds_field_c.len);
  capi::BorrowedFieldsWithBounds diplomat_raw_struct_out_value_bounds2 = diplomat_raw_struct_out_value.bounds2;
  capi::DiplomatU16StringView diplomat_slice_raw_out_value_bounds2_field_a = diplomat_raw_struct_out_value_bounds2.field_a;
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value_bounds2_field_a.data, diplomat_slice_raw_out_value_bounds2_field_a.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_bounds2_field_b = diplomat_raw_struct_out_value_bounds2.field_b;
  std::string_view str(diplomat_str_raw_out_value_bounds2_field_b.data, diplomat_str_raw_out_value_bounds2_field_b.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_bounds2_field_c = diplomat_raw_struct_out_value_bounds2.field_c;
  std::string_view str(diplomat_str_raw_out_value_bounds2_field_c.data, diplomat_str_raw_out_value_bounds2_field_c.len);
  return NestedBorrowedFields{ .fields = std::move(BorrowedFields{ .a = std::move(slice), .b = std::move(str), .c = std::move(str) }), .bounds = std::move(BorrowedFieldsWithBounds{ .field_a = std::move(slice), .field_b = std::move(str), .field_c = std::move(str) }), .bounds2 = std::move(BorrowedFieldsWithBounds{ .field_a = std::move(slice), .field_b = std::move(str), .field_c = std::move(str) }) };
}
#endif
