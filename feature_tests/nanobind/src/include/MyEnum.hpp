#ifndef SOMELIB_MyEnum_HPP
#define SOMELIB_MyEnum_HPP

#include "MyEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    int8_t MyEnum_into_value(somelib::capi::MyEnum self);

    somelib::capi::MyEnum MyEnum_get_a(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::capi::MyEnum somelib::MyEnum::AsFFI() const {
    return static_cast<somelib::capi::MyEnum>(value);
}

inline somelib::MyEnum somelib::MyEnum::FromFFI(somelib::capi::MyEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::MyEnum_A:
        case somelib::capi::MyEnum_B:
        case somelib::capi::MyEnum_C:
        case somelib::capi::MyEnum_D:
        case somelib::capi::MyEnum_E:
        case somelib::capi::MyEnum_F:
            return static_cast<somelib::MyEnum::Value>(c_enum);
        default:
            std::abort();
    }
}

inline int8_t somelib::MyEnum::into_value() const {
    auto result = somelib::capi::MyEnum_into_value(this->AsFFI());
    return result;
}

inline somelib::MyEnum somelib::MyEnum::get_a() {
    auto result = somelib::capi::MyEnum_get_a();
    return somelib::MyEnum::FromFFI(result);
}
#endif // SOMELIB_MyEnum_HPP
