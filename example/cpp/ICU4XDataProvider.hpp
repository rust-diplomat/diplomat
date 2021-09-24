#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <span>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XDataProvider.h"
}

class ICU4XDataProvider;

/**
 * A destruction policy for using ICU4XDataProvider with std::unique_ptr.
 */
struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:

  /**
   * Construct a [StaticDataProvider](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html).
   */
  static ICU4XDataProvider new_static();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  inline ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};


inline ICU4XDataProvider ICU4XDataProvider::new_static() {
  return ICU4XDataProvider(capi::ICU4XDataProvider_new_static());
}
#endif
