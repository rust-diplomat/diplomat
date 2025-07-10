#ifndef ScalarPairWithPadding_HPP
#define ScalarPairWithPadding_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {

    void ScalarPairWithPadding_assert_value(diplomat::capi::ScalarPairWithPadding self);

    } // extern "C"
} // namespace capi
} // namespace

inline void ScalarPairWithPadding::assert_value() const {
  diplomat::capi::ScalarPairWithPadding_assert_value(this->AsFFI());
}


inline diplomat::capi::ScalarPairWithPadding ScalarPairWithPadding::AsFFI() const {
  return diplomat::capi::ScalarPairWithPadding {
    /* .first = */ first,
    /* .second = */ second,
  };
}

inline ScalarPairWithPadding ScalarPairWithPadding::FromFFI(diplomat::capi::ScalarPairWithPadding c_struct) {
  return ScalarPairWithPadding {
    /* .first = */ c_struct.first,
    /* .second = */ c_struct.second,
  };
}


#endif // ScalarPairWithPadding_HPP
