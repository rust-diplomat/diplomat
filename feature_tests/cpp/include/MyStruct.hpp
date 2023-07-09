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

#include "MyStruct.h"

#include "MyEnum.hpp"
struct MyStruct;

struct MyStruct {
 public:
  uint8_t a;
  bool b;
  uint8_t c;
  uint64_t d;
  int32_t e;
  char32_t f;
  MyEnum g;
  static MyStruct new_();
  uint8_t into_a();
};


inline MyStruct MyStruct::new_() {
  capi::MyStruct diplomat_raw_struct_out_value = capi::MyStruct_new();
  return MyStruct{ .a = std::move(diplomat_raw_struct_out_value.a), .b = std::move(diplomat_raw_struct_out_value.b), .c = std::move(diplomat_raw_struct_out_value.c), .d = std::move(diplomat_raw_struct_out_value.d), .e = std::move(diplomat_raw_struct_out_value.e), .f = std::move(diplomat_raw_struct_out_value.f), .g = std::move(static_cast<MyEnum>(diplomat_raw_struct_out_value.g)) };
}
inline uint8_t MyStruct::into_a() {
  MyStruct diplomat_wrapped_struct_this = std::move(*this);
  return capi::MyStruct_into_a(capi::MyStruct{ .a = diplomat_wrapped_struct_this.a, .b = diplomat_wrapped_struct_this.b, .c = diplomat_wrapped_struct_this.c, .d = diplomat_wrapped_struct_this.d, .e = diplomat_wrapped_struct_this.e, .f = diplomat_wrapped_struct_this.f, .g = static_cast<capi::MyEnum>(diplomat_wrapped_struct_this.g) });
}
#endif
