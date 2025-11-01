#ifndef SOMELIB_ScalarPairWithPadding_HPP
#define SOMELIB_ScalarPairWithPadding_HPP

#include "ScalarPairWithPadding.d.hpp"

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

    void ScalarPairWithPadding_assert_value(somelib::capi::ScalarPairWithPadding self);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::ScalarPairWithPadding::assert_value() const {
    somelib::capi::ScalarPairWithPadding_assert_value(this->AsFFI());
}


inline somelib::capi::ScalarPairWithPadding somelib::ScalarPairWithPadding::AsFFI() const {
    return somelib::capi::ScalarPairWithPadding {
        /* .first = */ first,
        /* .second = */ second,
    };
}

inline somelib::ScalarPairWithPadding somelib::ScalarPairWithPadding::FromFFI(somelib::capi::ScalarPairWithPadding c_struct) {
    return somelib::ScalarPairWithPadding {
        /* .first = */ c_struct.first,
        /* .second = */ c_struct.second,
    };
}


#endif // SOMELIB_ScalarPairWithPadding_HPP
