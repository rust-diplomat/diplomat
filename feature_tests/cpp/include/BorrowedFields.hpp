#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "BorrowedFields.h"

class Bar;
struct BorrowedFields;

struct BorrowedFields {
 public:
  std::u16string_view a;
  std::string_view b;
  std::string_view c;

  /**
   * Lifetimes: `bar`, `dstr16`, `utf8_str` must live at least as long as the output.
   * 
   * 
   * Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).
   */
  static BorrowedFields from_bar_and_strings(const Bar& bar, const std::u16string_view dstr16, const std::string_view utf8_str);
};

#include "Bar.hpp"

inline BorrowedFields BorrowedFields::from_bar_and_strings(const Bar& bar, const std::u16string_view dstr16, const std::string_view utf8_str) {
  capi::BorrowedFields diplomat_raw_struct_out_value = capi::BorrowedFields_from_bar_and_strings(bar.AsFFI(), dstr16.data(), dstr16.size(), utf8_str.data(), utf8_str.size());
  capi::DiplomatString16View diplomat_slice_raw_out_value_a = diplomat_raw_struct_out_value.a;
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value_a.data, diplomat_slice_raw_out_value_a.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_b = diplomat_raw_struct_out_value.b;
  std::string_view str(diplomat_str_raw_out_value_b.data, diplomat_str_raw_out_value_b.len);
  capi::DiplomatStringView diplomat_str_raw_out_value_c = diplomat_raw_struct_out_value.c;
  std::string_view str(diplomat_str_raw_out_value_c.data, diplomat_str_raw_out_value_c.len);
  return BorrowedFields{ .a = std::move(slice), .b = std::move(str), .c = std::move(str) };
}
#endif
