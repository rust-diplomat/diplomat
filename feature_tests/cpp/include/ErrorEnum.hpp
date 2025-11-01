#ifndef SOMELIB_ErrorEnum_HPP
#define SOMELIB_ErrorEnum_HPP

#include "ErrorEnum.d.hpp"

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

} // namespace capi
} // namespace

inline somelib::capi::ErrorEnum somelib::ErrorEnum::AsFFI() const {
    return static_cast<somelib::capi::ErrorEnum>(value);
}

inline somelib::ErrorEnum somelib::ErrorEnum::FromFFI(somelib::capi::ErrorEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::ErrorEnum_Foo:
        case somelib::capi::ErrorEnum_Bar:
            return static_cast<somelib::ErrorEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_ErrorEnum_HPP
