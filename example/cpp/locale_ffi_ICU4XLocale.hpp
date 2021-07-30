#ifndef locale_ffi_ICU4XLocale_HPP
#define locale_ffi_ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "locale_ffi_ICU4XLocale.h"
}

class ICU4XLocale;

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  static ICU4XLocale new_(const std::string_view name);
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


ICU4XLocale ICU4XLocale::new_(const std::string_view name) {
  return ICU4XLocale(capi::ICU4XLocale_new(name.data(), name.length()));
}
#endif
