#ifndef ScalarPairWithPadding_D_HPP
#define ScalarPairWithPadding_D_HPP

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


/**
 * Testing JS-specific layout/padding behavior
 */
struct ScalarPairWithPadding {
  uint8_t first;
  uint32_t second;

  inline void assert_value() const;

  inline diplomat::capi::ScalarPairWithPadding AsFFI() const;
  inline static ScalarPairWithPadding FromFFI(diplomat::capi::ScalarPairWithPadding c_struct);
};


#endif // ScalarPairWithPadding_D_HPP
