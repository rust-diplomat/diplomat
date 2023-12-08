#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XDataProvider.h"

class ICU4XDataProvider;

/**
 * A destruction policy for using ICU4XDataProvider with std::unique_ptr.
 */
struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};

/**
 * An ICU4X data provider, capable of loading ICU4X data keys from some source.
 * 
 * See the [Rust documentation for `icu_provider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html) for more information.
 */
class ICU4XDataProvider {
 public:

  /**
   * See the [Rust documentation for `get_static_provider`](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html) for more information.
   */
  static ICU4XDataProvider new_static();

  /**
   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   */
  static diplomat::result<std::monostate, std::monostate> returns_result();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
  ICU4XDataProvider() = default;
  ICU4XDataProvider(ICU4XDataProvider&&) noexcept = default;
  ICU4XDataProvider& operator=(ICU4XDataProvider&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};


inline ICU4XDataProvider ICU4XDataProvider::new_static() {
  return ICU4XDataProvider(capi::ICU4XDataProvider_new_static());
}
inline diplomat::result<std::monostate, std::monostate> ICU4XDataProvider::returns_result() {
  auto diplomat_result_raw_out_value = capi::ICU4XDataProvider_returns_result();
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
#endif
