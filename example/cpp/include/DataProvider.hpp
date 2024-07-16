#ifndef DataProvider_HPP
#define DataProvider_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "DataProvider.h"

class DataProvider;

/**
 * A destruction policy for using DataProvider with std::unique_ptr.
 */
struct DataProviderDeleter {
  void operator()(capi::DataProvider* l) const noexcept {
    capi::icu4x_DataProvider_destroy_mv1(l);
  }
};

/**
 * An  data provider, capable of loading  data keys from some source.
 * 
 * See the [Rust documentation for `icu_provider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html) for more information.
 */
class DataProvider {
 public:

  /**
   * See the [Rust documentation for `get_static_provider`](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html) for more information.
   */
  static DataProvider new_static();

  /**
   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   */
  static diplomat::result<std::monostate, std::monostate> returns_result();
  inline const capi::DataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::DataProvider* AsFFIMut() { return this->inner.get(); }
  inline explicit DataProvider(capi::DataProvider* i) : inner(i) {}
  DataProvider() = default;
  DataProvider(DataProvider&&) noexcept = default;
  DataProvider& operator=(DataProvider&& other) noexcept = default;
 private:
  std::unique_ptr<capi::DataProvider, DataProviderDeleter> inner;
};


inline DataProvider DataProvider::new_static() {
  return DataProvider(capi::icu4x_DataProvider_new_static_mv1());
}
inline diplomat::result<std::monostate, std::monostate> DataProvider::returns_result() {
  auto diplomat_result_raw_out_value = capi::icu4x_DataProvider_returns_result_mv1();
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<std::monostate>(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<std::monostate>(std::monostate());
  }
  return diplomat_result_out_value;
}
#endif
