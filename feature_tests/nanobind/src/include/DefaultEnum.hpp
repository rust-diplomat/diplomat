#ifndef SOMELIB_DefaultEnum_HPP
#define SOMELIB_DefaultEnum_HPP

#include "DefaultEnum.d.hpp"

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

    somelib::capi::DefaultEnum DefaultEnum_new(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::capi::DefaultEnum somelib::DefaultEnum::AsFFI() const {
    return static_cast<somelib::capi::DefaultEnum>(value);
}

inline somelib::DefaultEnum somelib::DefaultEnum::FromFFI(somelib::capi::DefaultEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::DefaultEnum_A:
        case somelib::capi::DefaultEnum_B:
            return static_cast<somelib::DefaultEnum::Value>(c_enum);
        default:
            std::abort();
    }
}

inline somelib::DefaultEnum somelib::DefaultEnum::new_() {
    auto result = somelib::capi::DefaultEnum_new();
    return somelib::DefaultEnum::FromFFI(result);
}
#endif // SOMELIB_DefaultEnum_HPP
