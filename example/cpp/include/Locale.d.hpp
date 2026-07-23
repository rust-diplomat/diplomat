#ifndef ICU4X_Locale_D_HPP
#define ICU4X_Locale_D_HPP

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
namespace capi { struct Locale; }
class Locale;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct Locale;
    extern "C" {
    void icu4x_Locale_destroy_mv1(Locale* self);
    }
} // namespace capi
} // namespace

namespace icu4x {
class Locale;
using LocaleRef = icu4x::diplomat::Ref<Locale, const icu4x::capi::Locale>;
using LocaleRefMut = icu4x::diplomat::Ref<Locale, icu4x::capi::Locale>;

/**
 * An  Locale, capable of representing strings like `"en-US"`.
 *
 * See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
 */
class Locale : public icu4x::diplomat::OpaquePointer<Locale, icu4x::capi::Locale, icu4x::capi::icu4x_Locale_destroy_mv1> {
public:

  /**
   * Construct an {@link Locale} from a locale identifier represented as a string.
   */
  inline static icu4x::Locale new_(std::string_view name);

};

} // namespace
#endif // ICU4X_Locale_D_HPP
