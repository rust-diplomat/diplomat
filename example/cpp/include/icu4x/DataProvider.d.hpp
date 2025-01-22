#ifndef icu4x_DataProvider_D_HPP
#define icu4x_DataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
}


namespace icu4x {
namespace capi {
    struct DataProvider;
} // namespace capi
} // namespace

namespace icu4x {
class DataProvider {
public:

  inline static std::unique_ptr<icu4x::DataProvider> new_static();

  inline static diplomat::result<std::monostate, std::monostate> returns_result();

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
#endif // icu4x_DataProvider_D_HPP
