#ifndef SOMELIB_ns_RenamedDeprecatedEnum_HPP
#define SOMELIB_ns_RenamedDeprecatedEnum_HPP

#include "RenamedDeprecatedEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {

} // namespace capi
} // namespace

inline somelib::ns::capi::RenamedDeprecatedEnum somelib::ns::RenamedDeprecatedEnum::AsFFI() const {
    return static_cast<somelib::ns::capi::RenamedDeprecatedEnum>(value);
}

inline somelib::ns::RenamedDeprecatedEnum somelib::ns::RenamedDeprecatedEnum::FromFFI(somelib::ns::capi::RenamedDeprecatedEnum c_enum) {
    switch (c_enum) {
        case somelib::ns::capi::RenamedDeprecatedEnum_A:
            return static_cast<somelib::ns::RenamedDeprecatedEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_ns_RenamedDeprecatedEnum_HPP
