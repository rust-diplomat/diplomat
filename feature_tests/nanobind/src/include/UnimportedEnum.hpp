#ifndef SOMELIB_UnimportedEnum_HPP
#define SOMELIB_UnimportedEnum_HPP

#include "UnimportedEnum.d.hpp"

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

inline somelib::capi::UnimportedEnum somelib::UnimportedEnum::AsFFI() const {
    return static_cast<somelib::capi::UnimportedEnum>(value);
}

inline somelib::UnimportedEnum somelib::UnimportedEnum::FromFFI(somelib::capi::UnimportedEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::UnimportedEnum_A:
        case somelib::capi::UnimportedEnum_B:
        case somelib::capi::UnimportedEnum_C:
            return static_cast<somelib::UnimportedEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_UnimportedEnum_HPP
