#ifndef ns_RenamedDeprecatedEnum_HPP
#define ns_RenamedDeprecatedEnum_HPP

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


namespace ns {
namespace capi {

} // namespace capi
} // namespace

inline ns::capi::RenamedDeprecatedEnum ns::RenamedDeprecatedEnum::AsFFI() const {
    return static_cast<ns::capi::RenamedDeprecatedEnum>(value);
}

inline ns::RenamedDeprecatedEnum ns::RenamedDeprecatedEnum::FromFFI(ns::capi::RenamedDeprecatedEnum c_enum) {
    switch (c_enum) {
        case ns::capi::RenamedDeprecatedEnum_A:
            return static_cast<ns::RenamedDeprecatedEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ns_RenamedDeprecatedEnum_HPP
