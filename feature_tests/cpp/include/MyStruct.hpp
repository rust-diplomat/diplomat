#ifndef MyStruct_HPP
#define MyStruct_HPP

#include "MyStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "MyEnum.hpp"
#include "MyZst.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::MyStruct MyStruct_new(void);
    
    uint8_t MyStruct_into_a(diplomat::capi::MyStruct self);
    
    typedef struct MyStruct_returns_zst_result_result { bool is_ok;} MyStruct_returns_zst_result_result;
    MyStruct_returns_zst_result_result MyStruct_returns_zst_result(void);
    
    typedef struct MyStruct_fails_zst_result_result { bool is_ok;} MyStruct_fails_zst_result_result;
    MyStruct_fails_zst_result_result MyStruct_fails_zst_result(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline MyStruct MyStruct::new_() {
  auto result = diplomat::capi::MyStruct_new();
  return MyStruct::FromFFI(result);
}

inline uint8_t MyStruct::into_a() {
  auto result = diplomat::capi::MyStruct_into_a(this->AsFFI());
  return result;
}

inline diplomat::result<std::monostate, MyZst> MyStruct::returns_zst_result() {
  auto result = diplomat::capi::MyStruct_returns_zst_result();
  return result.is_ok ? diplomat::result<std::monostate, MyZst>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, MyZst>(diplomat::Err<MyZst>(MyZst {}));
}

inline diplomat::result<std::monostate, MyZst> MyStruct::fails_zst_result() {
  auto result = diplomat::capi::MyStruct_fails_zst_result();
  return result.is_ok ? diplomat::result<std::monostate, MyZst>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, MyZst>(diplomat::Err<MyZst>(MyZst {}));
}


inline diplomat::capi::MyStruct MyStruct::AsFFI() const {
  return diplomat::capi::MyStruct {
    /* .a = */ a,
    /* .b = */ b,
    /* .c = */ c,
    /* .d = */ d,
    /* .e = */ e,
    /* .f = */ f,
    /* .g = */ g.AsFFI(),
  };
}

inline MyStruct MyStruct::FromFFI(diplomat::capi::MyStruct c_struct) {
  return MyStruct {
    /* .a = */ c_struct.a,
    /* .b = */ c_struct.b,
    /* .c = */ c_struct.c,
    /* .d = */ c_struct.d,
    /* .e = */ c_struct.e,
    /* .f = */ c_struct.f,
    /* .g = */ MyEnum::FromFFI(c_struct.g),
  };
}


#endif // MyStruct_HPP
