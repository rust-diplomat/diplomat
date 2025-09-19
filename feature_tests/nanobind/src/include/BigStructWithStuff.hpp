#ifndef BigStructWithStuff_HPP
#define BigStructWithStuff_HPP

#include "BigStructWithStuff.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ScalarPairWithPadding.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void BigStructWithStuff_assert_value(diplomat::capi::BigStructWithStuff self, uint16_t extra_val);

    void BigStructWithStuff_assert_slice(diplomat::capi::DiplomatBigStructWithStuffView slice, uint16_t second_value);

    } // extern "C"
} // namespace capi
} // namespace

inline void BigStructWithStuff::assert_value(uint16_t extra_val) const {
    diplomat::capi::BigStructWithStuff_assert_value(this->AsFFI(),
        extra_val);
}

inline void BigStructWithStuff::assert_slice(diplomat::span<const BigStructWithStuff> slice, uint16_t second_value) {
    diplomat::capi::BigStructWithStuff_assert_slice({reinterpret_cast<const diplomat::capi::BigStructWithStuff*>(slice.data()), slice.size()},
        second_value);
}


inline diplomat::capi::BigStructWithStuff BigStructWithStuff::AsFFI() const {
    return diplomat::capi::BigStructWithStuff {
        /* .first = */ first,
        /* .second = */ second,
        /* .third = */ third,
        /* .fourth = */ fourth.AsFFI(),
        /* .fifth = */ fifth,
    };
}

inline BigStructWithStuff BigStructWithStuff::FromFFI(diplomat::capi::BigStructWithStuff c_struct) {
    return BigStructWithStuff {
        /* .first = */ c_struct.first,
        /* .second = */ c_struct.second,
        /* .third = */ c_struct.third,
        /* .fourth = */ ScalarPairWithPadding::FromFFI(c_struct.fourth),
        /* .fifth = */ c_struct.fifth,
    };
}


#endif // BigStructWithStuff_HPP
