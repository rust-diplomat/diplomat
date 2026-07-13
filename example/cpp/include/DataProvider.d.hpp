#ifndef ICU4X_DataProvider_D_HPP
#define ICU4X_DataProvider_D_HPP

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
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct DataProvider;
    extern "C" {
    void icu4x_DataProvider_destroy_mv1(DataProvider* self);
    }
} // namespace capi
} // namespace

namespace icu4x {
class DataProvider;
using DataProviderRef = icu4x::diplomat::Ref<DataProvider, const icu4x::capi::DataProvider>;
using DataProviderRefMut = icu4x::diplomat::Ref<DataProvider, icu4x::capi::DataProvider>;

/**
 * An  data provider, capable of loading  data keys from some source.
 *
 * See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
 */
class DataProvider : public icu4x::diplomat::OpaquePointer<DataProvider, icu4x::capi::DataProvider, icu4x::capi::icu4x_DataProvider_destroy_mv1> {
public:

  /**
   * See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
   */
  inline static icu4x::DataProvider new_static();

  /**
   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   */
  inline static icu4x::diplomat::result<std::monostate, std::monostate> returns_result();

};

} // namespace
#endif // ICU4X_DataProvider_D_HPP
