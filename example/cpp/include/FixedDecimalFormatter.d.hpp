#ifndef ICU4X_FixedDecimalFormatter_D_HPP
#define ICU4X_FixedDecimalFormatter_D_HPP

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
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct FixedDecimal; }
class FixedDecimal;
namespace capi { struct FixedDecimalFormatter; }
class FixedDecimalFormatter;
namespace capi { struct Locale; }
class Locale;
struct FixedDecimalFormatterOptions;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct FixedDecimalFormatter;
    extern "C" {
    void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);
    }
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimalFormatter;
using FixedDecimalFormatterRef = icu4x::diplomat::Ref<FixedDecimalFormatter, const icu4x::capi::FixedDecimalFormatter>;
using FixedDecimalFormatterRefMut = icu4x::diplomat::Ref<FixedDecimalFormatter, icu4x::capi::FixedDecimalFormatter>;

/**
 * An  Fixed Decimal Format object, capable of formatting a {@link FixedDecimal} as a string.
 *
 * See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
 */
class FixedDecimalFormatter : public icu4x::diplomat::OpaquePointer<FixedDecimalFormatter, icu4x::capi::FixedDecimalFormatter, icu4x::capi::icu4x_FixedDecimalFormatter_destroy_mv1> {
public:

  /**
   * Creates a new {@link FixedDecimalFormatter} from locale data.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
   */
  inline static icu4x::diplomat::result<icu4x::FixedDecimalFormatter, std::monostate> try_new(const icu4x::Locale& locale, const icu4x::DataProvider& provider, icu4x::FixedDecimalFormatterOptions options);

  /**
   * Formats a {@link FixedDecimal} to a string.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
   */
  inline std::string format_write(const icu4x::FixedDecimal& value) const;
  template<typename W>
  inline void format_write_write(const icu4x::FixedDecimal& value, W& writeable_output) const;

};

} // namespace
#endif // ICU4X_FixedDecimalFormatter_D_HPP
