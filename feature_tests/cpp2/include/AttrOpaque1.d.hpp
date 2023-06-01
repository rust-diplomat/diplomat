#ifndef AttrOpaque1_D_HPP
#define AttrOpaque1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque1.d.h"


class AttrOpaque1 {
public:

  inline void method() const;

  inline const capi::AttrOpaque1* AsFFI() const;
  inline capi::AttrOpaque1* AsFFI();
  inline static const AttrOpaque1* FromFFI(const capi::AttrOpaque1* ptr);
  inline static AttrOpaque1* FromFFI(capi::AttrOpaque1* ptr);
  inline static void operator delete(void* ptr);
private:
  AttrOpaque1() = delete;
  AttrOpaque1(const AttrOpaque1&) = delete;
  AttrOpaque1(AttrOpaque1&&) noexcept = delete;
  AttrOpaque1 operator=(const AttrOpaque1&) = delete;
  AttrOpaque1 operator=(AttrOpaque1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // AttrOpaque1_D_HPP
