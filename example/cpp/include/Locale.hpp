#ifndef Locale_HPP
#define Locale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Locale.h"

class Locale;

/**
 * A destruction policy for using Locale with std::unique_ptr.
 */
struct LocaleDeleter {
  void operator()(capi::Locale* l) const noexcept {
    capi::icu4x_Locale_destroy_mv1(l);
  }
};

/**
 * An  Locale, capable of representing strings like `"en-US"`.
 * 
 * See the [Rust documentation for `Locale`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
 */
class Locale {
 public:

  /**
   * Construct an [`Locale`] from a locale identifier represented as a string.
   */
  static Locale new_(const std::string_view name);
  inline const capi::Locale* AsFFI() const { return this->inner.get(); }
  inline capi::Locale* AsFFIMut() { return this->inner.get(); }
  inline explicit Locale(capi::Locale* i) : inner(i) {}
  Locale() = default;
  Locale(Locale&&) noexcept = default;
  Locale& operator=(Locale&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Locale, LocaleDeleter> inner;
};


inline Locale Locale::new_(const std::string_view name) {
  return Locale(capi::icu4x_Locale_new_mv1(name.data(), name.size()));
}
#endif
