#ifndef BigStructWithStuff_D_HPP
#define BigStructWithStuff_D_HPP

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

struct ScalarPairWithPadding;


namespace diplomat {
namespace capi {
    struct BigStructWithStuff {
      uint8_t first;
      uint16_t second;
      uint16_t third;
      diplomat::capi::ScalarPairWithPadding fourth;
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


/**
 * Testing JS-specific layout/padding behavior
 * Also being used to test CPP backends taking structs with primitive values.
 */
struct BigStructWithStuff {
  uint8_t first;
  uint16_t second;
  uint16_t third;
  ScalarPairWithPadding fourth;
  uint8_t fifth;

  inline void assert_value(uint16_t extra_val) const;

  inline static void assert_slice(diplomat::span<const BigStructWithStuff> slice, uint16_t second_value);

  inline diplomat::capi::BigStructWithStuff AsFFI() const;
  inline static BigStructWithStuff FromFFI(diplomat::capi::BigStructWithStuff c_struct);
};


#endif // BigStructWithStuff_D_HPP
