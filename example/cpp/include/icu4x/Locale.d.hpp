#ifndef icu4x_Locale_D_HPP
#define icu4x_Locale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Locale; }
class Locale;
}


namespace icu4x {
namespace capi {
    struct Locale;
} // namespace capi
} // namespace

namespace icu4x {
class Locale {
public:

  inline static std::unique_ptr<icu4x::Locale> new_(std::string_view name);

  inline const icu4x::capi::Locale* AsFFI() const;
  inline icu4x::capi::Locale* AsFFI();
  inline static const icu4x::Locale* FromFFI(const icu4x::capi::Locale* ptr);
  inline static icu4x::Locale* FromFFI(icu4x::capi::Locale* ptr);
  inline static void operator delete(void* ptr);
private:
  Locale() = delete;
  Locale(const icu4x::Locale&) = delete;
  Locale(icu4x::Locale&&) noexcept = delete;
  Locale operator=(const icu4x::Locale&) = delete;
  Locale operator=(icu4x::Locale&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_Locale_D_HPP
