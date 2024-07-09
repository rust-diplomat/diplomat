#ifndef ICU4XLocale_D_HPP
#define ICU4XLocale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace icu4x {
namespace capi {typedef struct ICU4XLocale ICU4XLocale; }
class ICU4XLocale;
}


namespace icu4x {
namespace capi {
    typedef struct ICU4XLocale ICU4XLocale;
}

class ICU4XLocale {
public:

  inline static std::unique_ptr<icu4x::ICU4XLocale> new_(std::string_view name);

  inline const icu4x::capi::ICU4XLocale* AsFFI() const;
  inline icu4x::capi::ICU4XLocale* AsFFI();
  inline static const icu4x::ICU4XLocale* FromFFI(const icu4x::capi::ICU4XLocale* ptr);
  inline static icu4x::ICU4XLocale* FromFFI(icu4x::capi::ICU4XLocale* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocale() = delete;
  ICU4XLocale(const icu4x::ICU4XLocale&) = delete;
  ICU4XLocale(icu4x::ICU4XLocale&&) noexcept = delete;
  ICU4XLocale operator=(const icu4x::ICU4XLocale&) = delete;
  ICU4XLocale operator=(icu4x::ICU4XLocale&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

}
#endif // ICU4XLocale_D_HPP
