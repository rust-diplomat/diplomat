#ifndef SOMELIB_BigStructWithStuff_D_HPP
#define SOMELIB_BigStructWithStuff_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ScalarPairWithPadding.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct BigStructWithStuff;
struct ScalarPairWithPadding;
} // namespace somelib



namespace somelib {
namespace capi {
    struct BigStructWithStuff {
      uint8_t first;
      uint16_t second;
      uint16_t third;
      somelib::capi::ScalarPairWithPadding fourth;
      uint8_t fifth;
    };

    typedef struct BigStructWithStuff_option {union { BigStructWithStuff ok; }; bool is_ok; } BigStructWithStuff_option;
    typedef struct DiplomatBigStructWithStuffView {
      const BigStructWithStuff* data;
      size_t len;
    } DiplomatBigStructWithStuffView;

    typedef struct DiplomatBigStructWithStuffViewMut {
      BigStructWithStuff* data;
      size_t len;
    } DiplomatBigStructWithStuffViewMut;
} // namespace capi
} // namespace


namespace somelib {
/**
 * Testing JS-specific layout/padding behavior
 * Also being used to test CPP backends taking structs with primitive values.
 */
struct BigStructWithStuff {
    uint8_t first;
    uint16_t second;
    uint16_t third;
    somelib::ScalarPairWithPadding fourth;
    uint8_t fifth;

  inline void assert_value(uint16_t extra_val) const;

  inline static void assert_slice(somelib::diplomat::span<const somelib::BigStructWithStuff> slice, uint16_t second_value);

    inline somelib::capi::BigStructWithStuff AsFFI() const;
    inline static somelib::BigStructWithStuff FromFFI(somelib::capi::BigStructWithStuff c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::BigStructWithStuff>>>> {
        using type = somelib::capi::DiplomatBigStructWithStuffView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::BigStructWithStuff>>>> {
        using type = somelib::capi::DiplomatBigStructWithStuffViewMut;
};
}
#endif // SOMELIB_BigStructWithStuff_D_HPP
