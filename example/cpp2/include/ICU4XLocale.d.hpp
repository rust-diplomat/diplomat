#ifndef ICU4XLocale_D_HPP
#define ICU4XLocale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocale.d.h"


class ICU4XLocale {
public:

  inline static std::unique_ptr<ICU4XLocale> new_(std::string_view name);

  inline static std::unique_ptr<ICU4XLocale> new_from_bytes(diplomat::span<const uint8_t> bytes);

  inline const capi::ICU4XLocale* AsFFI() const;
  inline capi::ICU4XLocale* AsFFI();
  inline static const ICU4XLocale* FromFFI(const capi::ICU4XLocale* ptr);
  inline static ICU4XLocale* FromFFI(capi::ICU4XLocale* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocale() = delete;
  ICU4XLocale(const ICU4XLocale&) = delete;
  ICU4XLocale(ICU4XLocale&&) noexcept = delete;
  ICU4XLocale operator=(const ICU4XLocale&) = delete;
  ICU4XLocale operator=(ICU4XLocale&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocale_D_HPP
