#ifndef SOMELIB_MyStruct_HPP
#define SOMELIB_MyStruct_HPP

#include "MyStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MyEnum.hpp"
#include "MyZst.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::MyStruct MyStruct_new(void);

    somelib::capi::MyStruct MyStruct_new_overload(int32_t i);

    void MyStruct_takes_mut(somelib::capi::MyStruct* self, somelib::capi::MyStruct* o);

    void MyStruct_takes_const(const somelib::capi::MyStruct* self, somelib::capi::MyStruct* o);

    uint8_t MyStruct_into_a(somelib::capi::MyStruct self);

    typedef struct MyStruct_returns_zst_result_result { bool is_ok;} MyStruct_returns_zst_result_result;
    MyStruct_returns_zst_result_result MyStruct_returns_zst_result(void);

    typedef struct MyStruct_fails_zst_result_result { bool is_ok;} MyStruct_fails_zst_result_result;
    MyStruct_fails_zst_result_result MyStruct_fails_zst_result(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::MyStruct somelib::MyStruct::new_() {
    auto result = somelib::capi::MyStruct_new();
    return somelib::MyStruct::FromFFI(result);
}

inline somelib::MyStruct somelib::MyStruct::new_overload(int32_t i) {
    auto result = somelib::capi::MyStruct_new_overload(i);
    return somelib::MyStruct::FromFFI(result);
}

inline void somelib::MyStruct::takes_mut(somelib::MyStruct& o) {
    auto thisDiplomatRefClone = this->AsFFI();
    auto oDiplomatRefClone = o.AsFFI();
    somelib::capi::MyStruct_takes_mut(&thisDiplomatRefClone,
        &oDiplomatRefClone);
    *this = somelib::MyStruct::FromFFI(thisDiplomatRefClone);
    o = somelib::MyStruct::FromFFI(oDiplomatRefClone);
}

inline void somelib::MyStruct::takes_const(somelib::MyStruct& o) const {
    auto thisDiplomatRefClone = this->AsFFI();
    auto oDiplomatRefClone = o.AsFFI();
    somelib::capi::MyStruct_takes_const(&thisDiplomatRefClone,
        &oDiplomatRefClone);
    o = somelib::MyStruct::FromFFI(oDiplomatRefClone);
}

inline uint8_t somelib::MyStruct::into_a() const {
    auto result = somelib::capi::MyStruct_into_a(this->AsFFI());
    return result;
}

inline somelib::diplomat::result<std::monostate, somelib::MyZst> somelib::MyStruct::returns_zst_result() {
    auto result = somelib::capi::MyStruct_returns_zst_result();
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::MyZst>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::MyZst>(somelib::diplomat::Err<somelib::MyZst>(somelib::MyZst {}));
}

inline somelib::diplomat::result<std::monostate, somelib::MyZst> somelib::MyStruct::fails_zst_result() {
    auto result = somelib::capi::MyStruct_fails_zst_result();
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::MyZst>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::MyZst>(somelib::diplomat::Err<somelib::MyZst>(somelib::MyZst {}));
}


inline somelib::capi::MyStruct somelib::MyStruct::AsFFI() const {
    return somelib::capi::MyStruct {
        /* .a = */ a,
        /* .b = */ b,
        /* .c = */ c,
        /* .d = */ d,
        /* .e = */ e,
        /* .f = */ f,
        /* .g = */ g.AsFFI(),
    };
}

inline somelib::MyStruct somelib::MyStruct::FromFFI(somelib::capi::MyStruct c_struct) {
    return somelib::MyStruct {
        /* .a = */ c_struct.a,
        /* .b = */ c_struct.b,
        /* .c = */ c_struct.c,
        /* .d = */ c_struct.d,
        /* .e = */ c_struct.e,
        /* .f = */ c_struct.f,
        /* .g = */ somelib::MyEnum::FromFFI(c_struct.g),
    };
}


#endif // SOMELIB_MyStruct_HPP
