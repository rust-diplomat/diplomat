#ifndef SOMELIB_ns_RenamedAttrEnum_HPP
#define SOMELIB_ns_RenamedAttrEnum_HPP

#include "RenamedAttrEnum.d.hpp"

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

inline somelib::ns::capi::RenamedAttrEnum somelib::ns::RenamedAttrEnum::AsFFI() const {
    return static_cast<somelib::ns::capi::RenamedAttrEnum>(value);
}

inline somelib::ns::RenamedAttrEnum somelib::ns::RenamedAttrEnum::FromFFI(somelib::ns::capi::RenamedAttrEnum c_enum) {
    switch (c_enum) {
        case somelib::ns::capi::RenamedAttrEnum_A:
        case somelib::ns::capi::RenamedAttrEnum_B:
        case somelib::ns::capi::RenamedAttrEnum_C:
            return static_cast<somelib::ns::RenamedAttrEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_ns_RenamedAttrEnum_HPP
