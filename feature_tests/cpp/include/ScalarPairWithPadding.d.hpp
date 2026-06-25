#ifndef SOMELIB_ScalarPairWithPadding_D_HPP
#define SOMELIB_ScalarPairWithPadding_D_HPP

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
    struct ScalarPairWithPadding {
      uint8_t first;
      uint32_t second;
    };

    typedef struct ScalarPairWithPadding_option {union { ScalarPairWithPadding ok; }; bool is_ok; } ScalarPairWithPadding_option;
} // namespace capi
} // namespace


namespace somelib {
/**
 * Testing JS-specific layout/padding behavior
 */
struct ScalarPairWithPadding {
    uint8_t first;
    uint32_t second;

  inline void assert_value() const;

    inline somelib::capi::ScalarPairWithPadding AsFFI() const;
    inline static somelib::ScalarPairWithPadding FromFFI(somelib::capi::ScalarPairWithPadding c_struct);
};

} // namespace
#endif // SOMELIB_ScalarPairWithPadding_D_HPP
