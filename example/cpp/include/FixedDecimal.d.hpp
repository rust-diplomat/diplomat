#ifndef ICU4X_FixedDecimal_D_HPP
#define ICU4X_FixedDecimal_D_HPP

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
namespace capi { struct FixedDecimal; }
class FixedDecimal;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct FixedDecimal;
    extern "C" {
    void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);
    }
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimal;
using FixedDecimalRef = icu4x::diplomat::Ref<FixedDecimal, const icu4x::capi::FixedDecimal>;
using FixedDecimalRefMut = icu4x::diplomat::Ref<FixedDecimal, icu4x::capi::FixedDecimal>;

/**
 * See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
 */
class FixedDecimal : public icu4x::diplomat::OpaquePointer<FixedDecimal, icu4x::capi::FixedDecimal, icu4x::capi::icu4x_FixedDecimal_destroy_mv1> {
public:

  /**
   * Construct an {@link FixedDecimal} from an integer.
   */
  inline static icu4x::FixedDecimal new_(int32_t v);

  /**
   * Multiply the {@link FixedDecimal} by a given power of ten.
   *
   * See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
   */
  inline void multiply_pow10(int16_t power);

  /**
   * Format the {@link FixedDecimal} as a string.
   *
   * See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  inline icu4x::diplomat::result<std::string, std::monostate> to_string() const;
  template<typename W>
  inline icu4x::diplomat::result<std::monostate, std::monostate> to_string_write(W& writeable_output) const;

};

} // namespace
#endif // ICU4X_FixedDecimal_D_HPP
