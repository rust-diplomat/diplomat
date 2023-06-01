#ifndef AttrOpaque2_D_HPP
#define AttrOpaque2_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque2.d.h"


class AttrOpaque2 {
public:

  inline const capi::AttrOpaque2* AsFFI() const;
  inline capi::AttrOpaque2* AsFFI();
  inline static const AttrOpaque2* FromFFI(const capi::AttrOpaque2* ptr);
  inline static AttrOpaque2* FromFFI(capi::AttrOpaque2* ptr);
  inline static void operator delete(void* ptr);
private:
  AttrOpaque2() = delete;
  AttrOpaque2(const AttrOpaque2&) = delete;
  AttrOpaque2(AttrOpaque2&&) noexcept = delete;
  AttrOpaque2 operator=(const AttrOpaque2&) = delete;
  AttrOpaque2 operator=(AttrOpaque2&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // AttrOpaque2_D_HPP
