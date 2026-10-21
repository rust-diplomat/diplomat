#ifndef ICU4X_FixedDecimalGroupingStrategy_HPP
#define ICU4X_FixedDecimalGroupingStrategy_HPP

#include "FixedDecimalGroupingStrategy.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    bool icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(icu4x::capi::FixedDecimalGroupingStrategy self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::FixedDecimalGroupingStrategy icu4x::FixedDecimalGroupingStrategy::AsFFI() const {
    return static_cast<icu4x::capi::FixedDecimalGroupingStrategy>(value);
}

inline icu4x::FixedDecimalGroupingStrategy icu4x::FixedDecimalGroupingStrategy::FromFFI(icu4x::capi::FixedDecimalGroupingStrategy c_enum) {
    switch (c_enum) {
        case icu4x::capi::FixedDecimalGroupingStrategy_Auto:
        case icu4x::capi::FixedDecimalGroupingStrategy_Never:
        case icu4x::capi::FixedDecimalGroupingStrategy_Always:
        case icu4x::capi::FixedDecimalGroupingStrategy_Min2:
            return static_cast<icu4x::FixedDecimalGroupingStrategy::Value>(c_enum);
        default:
            std::abort();
    }
}

inline bool icu4x::FixedDecimalGroupingStrategy::groups_digits() const {
    auto result = icu4x::capi::icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(this->AsFFI());
    return result;
}
#endif // ICU4X_FixedDecimalGroupingStrategy_HPP
