#ifndef icu4x_FixedDecimal_D_HPP
#define icu4x_FixedDecimal_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct FixedDecimal; }
class FixedDecimal;
}


namespace icu4x {
namespace capi {
    struct FixedDecimal;
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimal {
public:

  inline static std::unique_ptr<icu4x::FixedDecimal> new_(int32_t v);

  inline void multiply_pow10(int16_t power);

  inline diplomat::result<std::string, std::monostate> to_string() const;

  inline const icu4x::capi::FixedDecimal* AsFFI() const;
  inline icu4x::capi::FixedDecimal* AsFFI();
  inline static const icu4x::FixedDecimal* FromFFI(const icu4x::capi::FixedDecimal* ptr);
  inline static icu4x::FixedDecimal* FromFFI(icu4x::capi::FixedDecimal* ptr);
  inline static void operator delete(void* ptr);
private:
  FixedDecimal() = delete;
  FixedDecimal(const icu4x::FixedDecimal&) = delete;
  FixedDecimal(icu4x::FixedDecimal&&) noexcept = delete;
  FixedDecimal operator=(const icu4x::FixedDecimal&) = delete;
  FixedDecimal operator=(icu4x::FixedDecimal&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_FixedDecimal_D_HPP
