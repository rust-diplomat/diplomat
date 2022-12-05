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
};


inline MyStruct MyStruct::new_() {
  capi::MyStruct diplomat_raw_struct_out_value = capi::MyStruct_new();
  return MyStruct{ .a = std::move(diplomat_raw_struct_out_value.a), .b = std::move(diplomat_raw_struct_out_value.b), .c = std::move(diplomat_raw_struct_out_value.c), .d = std::move(diplomat_raw_struct_out_value.d), .e = std::move(diplomat_raw_struct_out_value.e), .f = std::move(diplomat_raw_struct_out_value.f), .g = std::move(static_cast<MyEnum>(diplomat_raw_struct_out_value.g)) };
}
#endif
