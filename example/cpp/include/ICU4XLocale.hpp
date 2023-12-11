#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLocale.h"

class ICU4XLocale;

/**
 * A destruction policy for using ICU4XLocale with std::unique_ptr.
 */
struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};

/**
 * An ICU4X Locale, capable of representing strings like `"en-US"`.
 * 
 * See the [Rust documentation for `Locale`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
 */
class ICU4XLocale {
 public:

  /**
   * Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
   */
  static ICU4XLocale new_(const std::string_view name);
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
  ICU4XLocale() = default;
  ICU4XLocale(ICU4XLocale&&) noexcept = default;
  ICU4XLocale& operator=(ICU4XLocale&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


inline ICU4XLocale ICU4XLocale::new_(const std::string_view name) {
  return ICU4XLocale(capi::ICU4XLocale_new(name.data(), name.size()));
}
#endif
