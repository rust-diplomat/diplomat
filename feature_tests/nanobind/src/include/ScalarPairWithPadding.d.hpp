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
    typedef struct DiplomatScalarPairWithPaddingView {
      const ScalarPairWithPadding* data;
      size_t len;
    } DiplomatScalarPairWithPaddingView;

    typedef struct DiplomatScalarPairWithPaddingViewMut {
      ScalarPairWithPadding* data;
      size_t len;
    } DiplomatScalarPairWithPaddingViewMut;
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
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::ScalarPairWithPadding>>>> {
        using type = somelib::capi::DiplomatScalarPairWithPaddingView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::ScalarPairWithPadding>>>> {
        using type = somelib::capi::DiplomatScalarPairWithPaddingViewMut;
};
}
#endif // SOMELIB_ScalarPairWithPadding_D_HPP
