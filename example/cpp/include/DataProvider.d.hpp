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
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An  data provider, capable of loading  data keys from some source.
 *
 * See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
 */
class DataProvider {
public:

  /**
   * See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
   */
  inline static std::unique_ptr<icu4x::DataProvider> new_static();

  /**
   * This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
   */
  inline static icu4x::diplomat::result<std::monostate, std::monostate> returns_result();

    inline const icu4x::capi::DataProvider* AsFFI() const;
    inline icu4x::capi::DataProvider* AsFFI();
    inline static const icu4x::DataProvider* FromFFI(const icu4x::capi::DataProvider* ptr);
    inline static icu4x::DataProvider* FromFFI(icu4x::capi::DataProvider* ptr);
    inline static void operator delete(void* ptr);
private:
    DataProvider() = delete;
    DataProvider(const icu4x::DataProvider&) = delete;
    DataProvider(icu4x::DataProvider&&) noexcept = delete;
    DataProvider operator=(const icu4x::DataProvider&) = delete;
    DataProvider operator=(icu4x::DataProvider&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_DataProvider_D_HPP
