#ifndef ICU4XDataProvider_D_HPP
#define ICU4XDataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct ICU4XDataProvider; }
class ICU4XDataProvider;
}


namespace icu4x {
namespace capi {
    struct ICU4XDataProvider;
} // namespace capi
} // namespace

namespace icu4x {
class ICU4XDataProvider {
public:

  inline static std::unique_ptr<icu4x::ICU4XDataProvider> new_static();

  inline static diplomat::result<std::monostate, std::monostate> returns_result();

  inline const icu4x::capi::ICU4XDataProvider* AsFFI() const;
  inline icu4x::capi::ICU4XDataProvider* AsFFI();
  inline static const icu4x::ICU4XDataProvider* FromFFI(const icu4x::capi::ICU4XDataProvider* ptr);
  inline static icu4x::ICU4XDataProvider* FromFFI(icu4x::capi::ICU4XDataProvider* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDataProvider() = delete;
  ICU4XDataProvider(const icu4x::ICU4XDataProvider&) = delete;
  ICU4XDataProvider(icu4x::ICU4XDataProvider&&) noexcept = delete;
  ICU4XDataProvider operator=(const icu4x::ICU4XDataProvider&) = delete;
  ICU4XDataProvider operator=(icu4x::ICU4XDataProvider&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4XDataProvider_D_HPP
