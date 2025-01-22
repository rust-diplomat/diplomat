#ifndef MyStruct_D_HPP
#define MyStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "MyEnum.d.hpp"
#include "diplomat_runtime.hpp"

struct MyZst;
class MyEnum;


namespace diplomat {
namespace capi {
    struct MyStruct {
      uint8_t a;
      bool b;
      uint8_t c;
      uint64_t d;
      int32_t e;
      char32_t f;
      diplomat::capi::MyEnum g;
    };
    
    typedef struct MyStruct_option {union { MyStruct ok; }; bool is_ok; } MyStruct_option;
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

  inline static diplomat::result<std::monostate, MyZst> fails_zst_result();

  inline diplomat::capi::MyStruct AsFFI() const;
  inline static MyStruct FromFFI(diplomat::capi::MyStruct c_struct);
};


#endif // MyStruct_D_HPP
