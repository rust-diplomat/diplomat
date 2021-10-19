#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include <span>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLocale.h"
}

class ICU4XLocale;

/**
 * A destruction policy for using ICU4XLocale with std::unique_ptr.
 */
struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:

  /**
   * Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
   */
  static ICU4XLocale new_(const std::string_view name);

  /**
   * Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
   */
  static ICU4XLocale new_from_bytes(const std::span<uint8_t> bytes);
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


inline ICU4XLocale ICU4XLocale::new_(const std::string_view name) {
  return ICU4XLocale(capi::ICU4XLocale_new(name.data(), name.size()));
}
inline ICU4XLocale ICU4XLocale::new_from_bytes(const std::span<uint8_t> bytes) {
  return ICU4XLocale(capi::ICU4XLocale_new_from_bytes(bytes.data(), bytes.size()));
}
#endif
