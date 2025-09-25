#ifndef SOMELIB_ContiguousEnum_HPP
#define SOMELIB_ContiguousEnum_HPP

#include "ContiguousEnum.d.hpp"

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

inline somelib::capi::ContiguousEnum somelib::ContiguousEnum::AsFFI() const {
    return static_cast<somelib::capi::ContiguousEnum>(value);
}

inline somelib::ContiguousEnum somelib::ContiguousEnum::FromFFI(somelib::capi::ContiguousEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::ContiguousEnum_C:
        case somelib::capi::ContiguousEnum_D:
        case somelib::capi::ContiguousEnum_E:
        case somelib::capi::ContiguousEnum_F:
            return static_cast<somelib::ContiguousEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_ContiguousEnum_HPP
