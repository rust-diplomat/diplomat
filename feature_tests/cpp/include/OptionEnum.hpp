#ifndef SOMELIB_OptionEnum_HPP
#define SOMELIB_OptionEnum_HPP

#include "OptionEnum.d.hpp"

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

inline somelib::capi::OptionEnum somelib::OptionEnum::AsFFI() const {
    return static_cast<somelib::capi::OptionEnum>(value);
}

inline somelib::OptionEnum somelib::OptionEnum::FromFFI(somelib::capi::OptionEnum c_enum) {
    switch (c_enum) {
        case somelib::capi::OptionEnum_Foo:
        case somelib::capi::OptionEnum_Bar:
        case somelib::capi::OptionEnum_Baz:
            return static_cast<somelib::OptionEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_OptionEnum_HPP
