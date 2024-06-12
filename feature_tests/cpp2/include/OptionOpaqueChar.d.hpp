#ifndef OptionOpaqueChar_D_HPP
#define OptionOpaqueChar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct OptionOpaqueChar OptionOpaqueChar;
}

class OptionOpaqueChar {
public:

  inline void assert_char(char32_t ch) const;

  inline const capi::OptionOpaqueChar* AsFFI() const;
  inline capi::OptionOpaqueChar* AsFFI();
  inline static const OptionOpaqueChar* FromFFI(const capi::OptionOpaqueChar* ptr);
  inline static OptionOpaqueChar* FromFFI(capi::OptionOpaqueChar* ptr);
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
