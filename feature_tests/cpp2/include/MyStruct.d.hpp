#ifndef MyStruct_D_HPP
#define MyStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyEnum.d.hpp"

struct MyZst;
class MyEnum;


namespace diplomat {
namespace capi {
    typedef struct MyStruct {
      uint8_t a;
      bool b;
      uint8_t c;
      uint64_t d;
      int32_t e;
      char32_t f;
      diplomat::capi::MyEnum g;
    } MyStruct;
} // namespace capi
} // namespace


struct MyStruct {
  uint8_t a;
  bool b;
  uint8_t c;
  uint64_t d;
  int32_t e;
  char32_t f;
  MyEnum g;

  inline static MyStruct new_();

  inline uint8_t into_a();

  inline static diplomat::result<std::monostate, MyZst> returns_zst_result();

  inline diplomat::capi::MyStruct AsFFI() const;
  inline static MyStruct FromFFI(diplomat::capi::MyStruct c_struct);
};


#endif // MyStruct_D_HPP
