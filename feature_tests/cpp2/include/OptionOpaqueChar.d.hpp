#ifndef OptionOpaqueChar_D_HPP
#define OptionOpaqueChar_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "OptionOpaqueChar.d.h"




class OptionOpaqueChar {
public:

  inline void assert_char(char32_t ch) const;

  inline const capi::OptionOpaqueChar* AsFFI() const;
  inline capi::OptionOpaqueChar* AsFFI();
  inline static const OptionOpaqueChar* FromFFI(const capi::OptionOpaqueChar* ptr);
  inline static OptionOpaqueChar* FromFFI(capi::OptionOpaqueChar* ptr);
  inline ~OptionOpaqueChar();
private:
  OptionOpaqueChar() = delete;
};





#endif // OptionOpaqueChar_D_HPP
