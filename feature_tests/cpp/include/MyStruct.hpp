#ifndef MyStruct_HPP
#define MyStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "MyStruct.h"
}

struct MyStruct;

/**
 * A destruction policy for using MyStruct with std::unique_ptr.
 */
struct MyStructDeleter {
  void operator()(capi::MyStruct* l) const noexcept {
    capi::MyStruct_destroy(l);
  }
};
struct MyStruct {
 public:
  uint8_t a;
  bool b;
  uint8_t c;
  uint64_t d;
  int32_t e;
  char32_t f;
  std::string_view g;
  static MyStruct new_(const std::string_view s);
};


inline MyStruct MyStruct::new_(const std::string_view s) {
  capi::MyStruct diplomat_raw_struct_out_value = capi::MyStruct_new(s.data(), s.size());
  capi::DiplomatStringView diplomat_str_raw_out_value_g = diplomat_raw_struct_out_value.g;
  std::string_view str(diplomat_str_raw_out_value_g.data, diplomat_str_raw_out_value_g.len);
  return MyStruct{ .a = std::move(diplomat_raw_struct_out_value.a), .b = std::move(diplomat_raw_struct_out_value.b), .c = std::move(diplomat_raw_struct_out_value.c), .d = std::move(diplomat_raw_struct_out_value.d), .e = std::move(diplomat_raw_struct_out_value.e), .f = std::move(diplomat_raw_struct_out_value.f), .g = std::move(str) };
}
#endif
