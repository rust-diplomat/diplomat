#ifndef SOMELIB_BigStructWithStuff_HPP
#define SOMELIB_BigStructWithStuff_HPP

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


namespace somelib {
namespace capi {
    extern "C" {

    void BigStructWithStuff_assert_value(somelib::capi::BigStructWithStuff self, uint16_t extra_val);

    void BigStructWithStuff_assert_slice(somelib::capi::DiplomatBigStructWithStuffView slice, uint16_t second_value);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::BigStructWithStuff::assert_value(uint16_t extra_val) const {
    somelib::capi::BigStructWithStuff_assert_value(this->AsFFI(),
        extra_val);
}

inline void somelib::BigStructWithStuff::assert_slice(somelib::diplomat::span<const somelib::BigStructWithStuff> slice, uint16_t second_value) {
    somelib::capi::BigStructWithStuff_assert_slice({reinterpret_cast<const somelib::capi::BigStructWithStuff*>(slice.data()), slice.size()},
        second_value);
}


inline somelib::capi::BigStructWithStuff somelib::BigStructWithStuff::AsFFI() const {
    return somelib::capi::BigStructWithStuff {
        /* .first = */ first,
        /* .second = */ second,
        /* .third = */ third,
        /* .fourth = */ fourth.AsFFI(),
        /* .fifth = */ fifth,
    };
}

inline somelib::BigStructWithStuff somelib::BigStructWithStuff::FromFFI(somelib::capi::BigStructWithStuff c_struct) {
    return somelib::BigStructWithStuff {
        /* .first = */ c_struct.first,
        /* .second = */ c_struct.second,
        /* .third = */ c_struct.third,
        /* .fourth = */ somelib::ScalarPairWithPadding::FromFFI(c_struct.fourth),
        /* .fifth = */ c_struct.fifth,
    };
}


#endif // SOMELIB_BigStructWithStuff_HPP
