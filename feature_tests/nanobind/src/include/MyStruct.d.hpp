#ifndef SOMELIB_MyStruct_D_HPP
#define SOMELIB_MyStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MyEnum.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct MyStruct;
struct MyZst;
class ContiguousEnum;
class MyEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MyStruct {
      uint8_t a;
      bool b;
      uint8_t c;
      uint64_t d;
      int32_t e;
      char32_t f;
      somelib::capi::MyEnum g;
    };

    typedef struct MyStruct_option {union { MyStruct ok; }; bool is_ok; } MyStruct_option;
    typedef struct DiplomatMyStructView {
      const MyStruct* data;
      size_t len;
    } DiplomatMyStructView;

    typedef struct DiplomatMyStructViewMut {
      MyStruct* data;
      size_t len;
    } DiplomatMyStructViewMut;
} // namespace capi
} // namespace


namespace somelib {
struct MyStruct {
    uint8_t a;
    bool b;
    uint8_t c;
    uint64_t d;
    int32_t e;
    char32_t f;
    somelib::MyEnum g;

  inline static somelib::MyStruct new_();

  inline static somelib::MyStruct new_overload(int32_t i);

  inline void takes_mut(somelib::MyStruct& o);

  inline void takes_const(somelib::MyStruct& o) const;

  inline uint8_t into_a() const;

  inline uint8_t take_ref_ret() const;

  inline static somelib::diplomat::result<std::monostate, somelib::MyZst> returns_zst_result();

  inline static somelib::diplomat::result<std::monostate, somelib::MyZst> fails_zst_result();

  inline static void assert_slice(somelib::diplomat::span<const somelib::MyStruct> slice, somelib::MyEnum second_value);

  inline static void assert_enum_slice(somelib::diplomat::span<const somelib::ContiguousEnum> slice, somelib::ContiguousEnum second_value);

    inline somelib::capi::MyStruct AsFFI() const;
    inline static somelib::MyStruct FromFFI(somelib::capi::MyStruct c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::MyStruct>>>> {
        using type = somelib::capi::DiplomatMyStructView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::MyStruct>>>> {
        using type = somelib::capi::DiplomatMyStructViewMut;
};
}
#endif // SOMELIB_MyStruct_D_HPP
