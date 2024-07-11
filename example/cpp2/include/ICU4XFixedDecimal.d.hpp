#ifndef ICU4XFixedDecimal_D_HPP
#define ICU4XFixedDecimal_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct ICU4XFixedDecimal; }
class ICU4XFixedDecimal;
}


namespace icu4x {
namespace capi {
    struct ICU4XFixedDecimal;
} // namespace capi
} // namespace

namespace icu4x {
class ICU4XFixedDecimal {
public:

  inline static std::unique_ptr<icu4x::ICU4XFixedDecimal> new_(int32_t v);

  inline void multiply_pow10(int16_t power);

  inline diplomat::result<std::string, std::monostate> to_string() const;

  inline const icu4x::capi::ICU4XFixedDecimal* AsFFI() const;
  inline icu4x::capi::ICU4XFixedDecimal* AsFFI();
  inline static const icu4x::ICU4XFixedDecimal* FromFFI(const icu4x::capi::ICU4XFixedDecimal* ptr);
  inline static icu4x::ICU4XFixedDecimal* FromFFI(icu4x::capi::ICU4XFixedDecimal* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimal() = delete;
  ICU4XFixedDecimal(const icu4x::ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal(icu4x::ICU4XFixedDecimal&&) noexcept = delete;
  ICU4XFixedDecimal operator=(const icu4x::ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal operator=(icu4x::ICU4XFixedDecimal&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4XFixedDecimal_D_HPP
