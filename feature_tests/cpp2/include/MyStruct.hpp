#ifndef MyStruct_HPP
#define MyStruct_HPP

#include "MyStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyEnum.hpp"
#include "MyStruct.h"


inline MyStruct MyStruct::new_() {
  auto result = capi::MyStruct_new();
  return MyStruct::FromFFI(result);
}

inline uint8_t MyStruct::into_a() {
  auto result = capi::MyStruct_into_a(this->AsFFI());
  return result;
}


inline capi::MyStruct MyStruct::AsFFI() const {
  return capi::MyStruct {
    .a = a,
    .b = b,
    .c = c,
    .d = d,
    .e = e,
    .f = f,
    .g = g.AsFFI(),
  };
}

inline MyStruct MyStruct::FromFFI(capi::MyStruct c_struct) {
  return MyStruct {
    .a = c_struct.a,
    .b = c_struct.b,
    .c = c_struct.c,
    .d = c_struct.d,
    .e = c_struct.e,
    .f = c_struct.f,
    .g = MyEnum::FromFFI(c_struct.g),
  };
}


#endif // MyStruct_HPP
