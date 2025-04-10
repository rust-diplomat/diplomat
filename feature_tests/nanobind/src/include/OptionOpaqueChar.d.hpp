#ifndef OptionOpaqueChar_D_HPP
#define OptionOpaqueChar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct OptionOpaqueChar;
} // namespace capi
} // namespace

class OptionOpaqueChar {
public:

  inline void assert_char(char32_t ch) const;

  inline const diplomat::capi::OptionOpaqueChar* AsFFI() const;
  inline diplomat::capi::OptionOpaqueChar* AsFFI();
  inline static const OptionOpaqueChar* FromFFI(const diplomat::capi::OptionOpaqueChar* ptr);
  inline static OptionOpaqueChar* FromFFI(diplomat::capi::OptionOpaqueChar* ptr);
  inline static void operator delete(void* ptr);
private:
  OptionOpaqueChar() = delete;
  OptionOpaqueChar(const OptionOpaqueChar&) = delete;
  OptionOpaqueChar(OptionOpaqueChar&&) noexcept = delete;
  OptionOpaqueChar operator=(const OptionOpaqueChar&) = delete;
  OptionOpaqueChar operator=(OptionOpaqueChar&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OptionOpaqueChar_D_HPP
